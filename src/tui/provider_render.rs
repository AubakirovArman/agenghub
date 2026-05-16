use crate::tui::ProviderPanel;

pub fn render_providers(out: &mut String, providers: &ProviderPanel) {
    push_line(out, "[Providers]");
    push_line(out, &format!("- default: {}", providers.default_provider));
    push_line(
        out,
        &format!(
            "- ready: {} | missing: {} | profiles: {}",
            providers.ready, providers.missing, providers.profiles
        ),
    );
    for status in &providers.statuses {
        let marker = if status.is_default { " default" } else { "" };
        let model = status
            .model
            .as_deref()
            .map(|value| format!(" model:{value}"))
            .unwrap_or_default();
        push_line(
            out,
            &format!(
                "- {} [{}{}] {}{}",
                status.id,
                status.state,
                marker,
                trim_line(&status.detail, 84),
                model
            ),
        );
    }
    render_roles(out, providers);
    push_line(out, "");
}

fn render_roles(out: &mut String, providers: &ProviderPanel) {
    push_line(out, "- roles:");
    for role in &providers.roles {
        let state = role
            .available
            .map(|ready| if ready { "ok" } else { "missing" })
            .unwrap_or("unknown");
        let fallback = if role.fallback.is_empty() {
            String::new()
        } else {
            format!(" fallback:{}", role.fallback.join(","))
        };
        push_line(
            out,
            &format!("  - {} -> {} ({state}){fallback}", role.role, role.provider),
        );
    }
}

fn trim_line(value: &str, max: usize) -> String {
    if value.len() <= max {
        return value.to_string();
    }
    format!("{}...", &value[..max])
}

fn push_line(out: &mut String, value: &str) {
    out.push_str(value);
    out.push('\n');
}
