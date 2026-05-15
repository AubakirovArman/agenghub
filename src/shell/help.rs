use super::commands::ShellMode;

pub(super) fn print(mode: ShellMode) {
    println!("current mode: {}", mode.as_str());
    println!("help or /help                show commands");
    println!("init                         initialize .agent");
    println!("mode plan|run                set plain-text behavior");
    println!("current                      show selected transaction");
    println!("close                        clear selected transaction");
    println!("chats                        list shell chat sessions");
    println!("chat [new|latest|id]         show, create, or select a chat");
    println!("messages                     print selected chat transcript");
    println!("sessions or history          list transactions");
    println!("doctor                       check local readiness");
    println!("providers [status|setup|test|diagnose]");
    println!("provider <id>                setup default provider");
    println!("config [show|set key value]  inspect or update config");
    println!("dashboard                    write local web dashboard");
    println!("open <tx-id|latest>          open report and select tx");
    println!("latest                       open latest transaction");
    println!("watch [tx-id|latest]         follow live transaction journal");
    println!("cancel [tx-id|latest]        request transaction cancellation");
    println!("approve [tx-id] <note>       record human approval/resolution");
    println!("resume [tx-id|latest]        resume a blocked transaction");
    println!("report [tx-id|latest]        print report");
    println!("effects [tx-id|latest]       print effect ledger");
    println!("explain [tx-id|latest]       explain failure/result and next steps");
    println!("memory [summary|audit]       show memory summary or audit");
    println!("skills [scorecard]           list skills or show scorecard");
    println!("undo [tx-id|last]            git revert a committed transaction");
    println!("ask <request>                write a draft spec");
    println!("do <request>                 write a draft and run it");
    println!("run <spec|request> [--no-commit]");
    println!("quit                         exit");
    println!("plain text                   plan mode: draft; run mode: execute");
    println!(
        "slash commands               /chats /chat latest /sessions /open latest /report /explain"
    );
}
