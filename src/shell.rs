mod actions;
mod chat;
mod chat_display;
mod commands;
mod help;
mod product;
mod run;

use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::agent_dir;
use commands::{parse_line, ShellCommand, ShellMode};

pub fn run(project_root: &Path) -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut current_tx: Option<String> = None;
    let mut current_chat = chat::create(project_root)?;
    let mut mode = ShellMode::Plan;
    writeln!(
        stdout,
        "AgentHub local shell. Type `help` for commands. Use `mode run` to execute plain text."
    )?;
    writeln!(stdout, "chat {}", current_chat.id)?;
    loop {
        prompt(&mut stdout, mode, current_tx.as_deref())?;
        let mut line = String::new();
        if stdin.lock().read_line(&mut line)? == 0 {
            break;
        }
        if !handle(
            project_root,
            parse_line(&line),
            &mut current_tx,
            &mut current_chat,
            &mut mode,
        )? {
            break;
        }
    }
    Ok(())
}

fn handle(
    root: &Path,
    command: ShellCommand,
    current_tx: &mut Option<String>,
    current_chat: &mut chat::ChatSession,
    mode: &mut ShellMode,
) -> Result<bool> {
    match command {
        ShellCommand::Empty => {}
        ShellCommand::Exit => return Ok(false),
        ShellCommand::Help => help::print(*mode),
        ShellCommand::Init => {
            agent_dir::init_project(root, false)?;
            println!("initialized {}", root.display());
        }
        ShellCommand::Current => actions::print_current(root, current_tx.as_deref())?,
        ShellCommand::Close => {
            *current_tx = None;
            println!("current session cleared");
        }
        ShellCommand::Mode(next) => update_mode(next, mode, current_chat)?,
        ShellCommand::Chats => chat_display::print_chats(root)?,
        ShellCommand::Chat(target) => update_chat(root, target.as_deref(), current_chat)?,
        ShellCommand::Messages => chat_display::print_messages(current_chat)?,
        ShellCommand::Sessions => actions::list_sessions(root)?,
        ShellCommand::Doctor => product::print_doctor(root)?,
        ShellCommand::Providers(args) => product::handle_providers(root, args.as_deref())?,
        ShellCommand::Config(args) => product::handle_config(root, args.as_deref())?,
        ShellCommand::Dashboard => product::open_dashboard(root)?,
        ShellCommand::Open(tx_id) => {
            let requested = (!tx_id.trim().is_empty()).then_some(tx_id.as_str());
            let opened = requested
                .map(|value| actions::resolve_tx(root, Some(value), current_tx.as_deref()))
                .unwrap_or_else(|| actions::latest_tx(root))?;
            actions::print_report(root, &opened)?;
            *current_tx = Some(opened);
        }
        ShellCommand::Watch(tx_id) => {
            let tx = actions::resolve_tx(root, tx_id.as_deref(), current_tx.as_deref())?;
            actions::watch_tx(root, &tx)?;
        }
        ShellCommand::Cancel(tx_id) => {
            let tx = actions::resolve_tx(root, tx_id.as_deref(), current_tx.as_deref())?;
            actions::cancel_tx(root, &tx)?;
        }
        ShellCommand::Report(tx_id) => {
            let tx = actions::resolve_tx(root, tx_id.as_deref(), current_tx.as_deref())?;
            actions::print_report(root, &tx)?;
        }
        ShellCommand::Effects(tx_id) => {
            let tx = actions::resolve_tx(root, tx_id.as_deref(), current_tx.as_deref())?;
            actions::print_effects(root, &tx)?;
        }
        ShellCommand::Explain(tx_id) => {
            let tx = actions::resolve_tx(root, tx_id.as_deref(), current_tx.as_deref())?;
            actions::print_explain(root, &tx)?;
        }
        ShellCommand::Memory(mode) => actions::print_memory(root, mode.as_deref())?,
        ShellCommand::Skills(mode) => actions::print_skills(root, mode.as_deref())?,
        ShellCommand::Undo(tx_id) => {
            let target = tx_id.unwrap_or_else(|| "last".to_string());
            *current_tx = Some(actions::undo_tx(root, &target)?);
        }
        ShellCommand::Ask(request) => {
            chat::append_user(current_chat, mode.as_str(), &request)?;
            let path = run::write_draft(root, &request)?;
            chat::append_draft(current_chat, &request, &path)?;
            println!("draft {}", path.display());
            println!("run {}  # execute", path.display());
        }
        ShellCommand::Do(request) => {
            chat::append_user(current_chat, mode.as_str(), &request)?;
            let tx_id = run::run_request(root, &request, false)?;
            chat::append_tx(current_chat, &request, &tx_id, &report_path(root, &tx_id))?;
            *current_tx = Some(tx_id);
        }
        ShellCommand::Run { target, no_commit } => {
            let path = run::resolve_run_target(root, &target)?;
            let tx_id = run::run_spec(root, &path, no_commit)?;
            chat::append_tx(current_chat, &target, &tx_id, &report_path(root, &tx_id))?;
            *current_tx = Some(tx_id);
        }
        ShellCommand::Message(request) => {
            handle_message(root, &request, *mode, current_tx, current_chat)?
        }
    }
    Ok(true)
}

fn prompt(stdout: &mut io::Stdout, mode: ShellMode, current_tx: Option<&str>) -> Result<()> {
    match current_tx {
        Some(tx) => write!(stdout, "agenthub:{}[{tx}]> ", mode.as_str())?,
        None => write!(stdout, "agenthub:{}> ", mode.as_str())?,
    }
    stdout.flush()?;
    Ok(())
}

fn handle_message(
    root: &Path,
    request: &str,
    mode: ShellMode,
    current_tx: &mut Option<String>,
    current_chat: &chat::ChatSession,
) -> Result<()> {
    chat::append_user(current_chat, mode.as_str(), request)?;
    match mode {
        ShellMode::Plan => {
            let path = run::write_draft(root, request)?;
            chat::append_draft(current_chat, request, &path)?;
            println!("draft {}", path.display());
            println!("mode run  # execute future plain text directly");
            println!("run {}  # execute this draft", path.display());
        }
        ShellMode::Run => {
            let tx_id = run::run_request(root, request, false)?;
            chat::append_tx(current_chat, request, &tx_id, &report_path(root, &tx_id))?;
            *current_tx = Some(tx_id);
        }
    }
    Ok(())
}

fn update_mode(
    next: Option<ShellMode>,
    mode: &mut ShellMode,
    current_chat: &chat::ChatSession,
) -> Result<()> {
    if let Some(next) = next {
        *mode = next;
    }
    chat::append_command(current_chat, "mode_changed", mode.as_str())?;
    println!("mode {}", mode.as_str());
    Ok(())
}

fn update_chat(
    root: &Path,
    target: Option<&str>,
    current_chat: &mut chat::ChatSession,
) -> Result<()> {
    match target.map(str::trim).filter(|value| !value.is_empty()) {
        Some("new") => *current_chat = chat::create(root)?,
        Some(target) => *current_chat = chat::open(root, target)?,
        None => {}
    }
    chat_display::print_summary(current_chat)
}

fn report_path(root: &Path, tx_id: &str) -> PathBuf {
    root.join(".agent").join("tx").join(tx_id).join("report.md")
}
