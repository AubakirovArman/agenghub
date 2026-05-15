use anyhow::{anyhow, Result};

use crate::aal::builder::build_spec;
use crate::aal::diagnostics::AalDiagnostic;
use crate::aal::draft::Draft;
use crate::aal::lexer::tokenize;
use crate::aal::section::{parse_section, Section};
use crate::aal::values::{join, parse_bool, parse_u16, parse_u32, parse_u64};
use crate::aal::AalParseOutput;
use crate::spec::RouteCheckSpec;

pub fn parse_aal(source: &str) -> Result<AalParseOutput> {
    let mut parser = AalParser::default();
    for (index, raw) in source.lines().enumerate() {
        parser.line(index + 1, raw)?;
    }
    parser.finish()
}

#[derive(Default)]
struct AalParser {
    draft: Draft,
    diagnostics: Vec<AalDiagnostic>,
    section: Section,
    open: bool,
    closed: bool,
}

impl AalParser {
    fn line(&mut self, line_number: usize, raw: &str) -> Result<()> {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            return Ok(());
        }
        if !self.open {
            return self.header(line_number, line);
        }
        if line == "}" {
            self.closed = true;
            self.open = false;
            return Ok(());
        }
        if let Some(section) = parse_section(line) {
            self.section = section;
            return Ok(());
        }
        let item = line.strip_prefix("- ").unwrap_or(line);
        let tokens = tokenize(item, line_number)?;
        if line.starts_with("- ") {
            return self.section_item(line_number, &tokens);
        }
        self.statement(line_number, &tokens)
    }

    fn header(&mut self, line_number: usize, line: &str) -> Result<()> {
        let tokens = tokenize(line, line_number)?;
        if tokens.len() == 3 && matches!(tokens[0].as_str(), "change" | "task") && tokens[2] == "{"
        {
            self.draft.name = Some(tokens[1].clone());
            self.open = true;
            return Ok(());
        }
        Err(anyhow!(
            "{}",
            AalDiagnostic::error(line_number, "expected `change Name {`").render()
        ))
    }

    fn statement(&mut self, line_number: usize, tokens: &[String]) -> Result<()> {
        if self.section == Section::Transaction {
            return self.transaction(line_number, tokens);
        }
        match tokens.first().map(String::as_str) {
            Some("workspace") if tokens.len() == 2 => {
                self.draft.workspace = Some(tokens[1].clone())
            }
            Some("goal") if tokens.len() >= 2 => self.draft.goal = Some(tokens[1..].join(" ")),
            Some("topology") if tokens.len() == 2 => self.draft.topology = Some(tokens[1].clone()),
            Some("use")
                if tokens.get(1).map(String::as_str) == Some("skill") && tokens.len() == 3 =>
            {
                self.draft.skills.push(tokens[2].clone())
            }
            Some(other) => return self.unknown(line_number, other),
            None => {}
        }
        Ok(())
    }

    fn section_item(&mut self, line_number: usize, tokens: &[String]) -> Result<()> {
        match self.section {
            Section::Allow => self.draft.allow.push(join(tokens)),
            Section::Deny => self.draft.deny.push(join(tokens)),
            Section::Rules => self.draft.rules.push(join(tokens)),
            Section::Execute => self.draft.execution_commands.push(join(tokens)),
            Section::Verify => self.verify(line_number, tokens)?,
            Section::Transaction => self.transaction(line_number, tokens)?,
            Section::Body => {
                return self.unknown(line_number, tokens.first().map_or("-", String::as_str))
            }
        }
        Ok(())
    }

    fn verify(&mut self, line_number: usize, tokens: &[String]) -> Result<()> {
        match tokens.first().map(String::as_str) {
            Some("profile") if tokens.len() == 2 => {
                self.draft.verify_profile = Some(tokens[1].clone())
            }
            Some("command") if tokens.len() >= 2 => {
                self.draft.verify_commands.push(join(&tokens[1..]))
            }
            Some("runtime_start") if tokens.len() >= 2 => {
                self.draft.runtime.start_command = Some(join(&tokens[1..]))
            }
            Some("runtime_base_url") if tokens.len() == 2 => {
                self.draft.runtime.base_url = Some(tokens[1].clone())
            }
            Some("runtime_timeout_secs") if tokens.len() == 2 => {
                self.draft.runtime.timeout_secs = Some(parse_u64(line_number, &tokens[1])?)
            }
            Some("runtime_smoke") => self.runtime_route(line_number, tokens)?,
            Some(_) => self.draft.verify_commands.push(join(tokens)),
            None => {}
        }
        Ok(())
    }

    fn runtime_route(&mut self, line_number: usize, tokens: &[String]) -> Result<()> {
        if tokens.len() == 5 && tokens[1] == "route" && tokens[3] == "expect" {
            self.draft.routes.push(RouteCheckSpec {
                path: tokens[2].clone(),
                expect: parse_u16(line_number, &tokens[4])?,
            });
            return Ok(());
        }
        self.unknown(line_number, "runtime_smoke")
    }

    fn transaction(&mut self, line_number: usize, tokens: &[String]) -> Result<()> {
        match tokens.first().map(String::as_str) {
            Some("isolation") if tokens.len() == 2 => {}
            Some("max_repair_attempts") if tokens.len() == 2 => {
                self.draft.transaction.max_repair_attempts = parse_u32(line_number, &tokens[1])?
            }
            Some("approval_required") if tokens.len() == 2 => {
                self.draft.transaction.approval_required = parse_bool(line_number, &tokens[1])?
            }
            Some("on_failure") if tokens.len() == 2 => {
                self.draft.transaction.rollback_on_failure = tokens[1] == "rollback"
            }
            Some("on_success") => self.on_success(&tokens[1..]),
            Some(other) => return self.unknown(line_number, other),
            None => {}
        }
        Ok(())
    }

    fn on_success(&mut self, tokens: &[String]) {
        self.draft.transaction.commit_on_success =
            tokens.iter().any(|token| token == "commit_code");
        self.draft.transaction.memory_promotion =
            if tokens.iter().any(|token| token == "promote_memory") {
                "on_success".to_string()
            } else {
                "never".to_string()
            };
    }

    fn finish(mut self) -> Result<AalParseOutput> {
        if !self.closed {
            return Err(anyhow!(
                "{}",
                AalDiagnostic::error(0, "missing closing `}`").render()
            ));
        }
        if !self.draft.routes.is_empty() && self.draft.runtime.start_command.is_none() {
            self.diagnostics.push(AalDiagnostic::warning(
                0,
                "runtime_smoke routes are recorded but not executed until runtime_start is set",
            ));
        }
        let spec = build_spec(&self.draft);
        spec.validate()?;
        Ok(AalParseOutput {
            spec,
            diagnostics: self.diagnostics,
        })
    }

    fn unknown<T>(&self, line_number: usize, item: &str) -> Result<T> {
        Err(anyhow!(
            "{}",
            AalDiagnostic::error(line_number, format!("unsupported AAL statement `{item}`"))
                .render()
        ))
    }
}
