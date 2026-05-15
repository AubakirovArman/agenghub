#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Section {
    #[default]
    Body,
    Allow,
    Deny,
    Rules,
    Verify,
    Execute,
    Transaction,
}

pub(crate) fn parse_section(line: &str) -> Option<Section> {
    match line {
        "allow:" | "allow edit:" => Some(Section::Allow),
        "deny:" | "deny edit:" => Some(Section::Deny),
        "rules:" => Some(Section::Rules),
        "verify:" => Some(Section::Verify),
        "execute:" | "execution:" => Some(Section::Execute),
        "transaction:" => Some(Section::Transaction),
        _ => None,
    }
}
