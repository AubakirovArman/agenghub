use anyhow::{anyhow, Result};

pub fn tokenize(line: &str, line_number: usize) -> Result<Vec<String>> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = line.chars().peekable();
    let mut quoted = false;

    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                quoted = !quoted;
                if !quoted {
                    tokens.push(std::mem::take(&mut current));
                    consume_space(&mut chars);
                }
            }
            '\\' if quoted => {
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            ch if ch.is_whitespace() && !quoted => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }

    if quoted {
        return Err(anyhow!(
            "error line {line_number}: unterminated quoted string"
        ));
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    Ok(tokens)
}

fn consume_space<I>(chars: &mut std::iter::Peekable<I>)
where
    I: Iterator<Item = char>,
{
    while chars.peek().is_some_and(|ch| ch.is_whitespace()) {
        chars.next();
    }
}
