use std::collections::BTreeSet;

use super::types::{ReadinessCheck, ReadinessGap, ReadinessGapCheck};

struct GapSpec {
    id: &'static str,
    label: &'static str,
    checks: &'static [&'static str],
}

const GAP_SPECS: &[GapSpec] = &[
    GapSpec {
        id: "shell_ux_aliases",
        label: "Shell UX alias evidence is missing or incomplete",
        checks: &["rc_check_shell_ux_aliases"],
    },
    GapSpec {
        id: "kimi",
        label: "Kimi provider, auth, or replacement-key evidence is incomplete",
        checks: &[
            "rc_check_kimi_unblock_rehearsal",
            "provider_kimi",
            "kimi_auth",
        ],
    },
    GapSpec {
        id: "dogfood",
        label: "RC dogfood thresholds or final gate are incomplete",
        checks: &[
            "real_sessions",
            "ops_flows",
            "project_edit_flows",
            "cost_receipts",
            "provider_deepseek",
            "provider_kimi",
            "open_blockers",
            "rc_dogfood_gate",
        ],
    },
    GapSpec {
        id: "latency",
        label: "Long-session latency evidence is missing or incomplete",
        checks: &["rc_check_long_session_latency"],
    },
    GapSpec {
        id: "approval",
        label: "Approval UX evidence is missing or incomplete",
        checks: &["rc_check_approval_ux"],
    },
];

pub(super) fn readiness_gaps(checks: &[ReadinessCheck]) -> Vec<ReadinessGap> {
    GAP_SPECS
        .iter()
        .filter_map(|spec| gap_from_spec(spec, checks))
        .collect()
}

fn gap_from_spec(spec: &GapSpec, checks: &[ReadinessCheck]) -> Option<ReadinessGap> {
    let gap_checks = spec
        .checks
        .iter()
        .filter_map(|id| checks.iter().find(|check| check.id == *id))
        .filter(|check| check.status != "passed")
        .map(|check| ReadinessGapCheck {
            id: check.id.clone(),
            status: check.status.clone(),
            detail: check.detail.clone(),
            blocker_kind: check.blocker_kind.clone(),
            next_commands: check.next_commands.clone(),
        })
        .collect::<Vec<_>>();

    if gap_checks.is_empty() {
        return None;
    }

    let status = if gap_checks.iter().any(|check| check.status == "blocked") {
        "blocked"
    } else {
        "missing"
    };
    let unresolved = gap_checks
        .iter()
        .map(|check| check.id.as_str())
        .collect::<Vec<_>>()
        .join(",");
    let blocker_kinds = gap_checks
        .iter()
        .filter_map(|check| check.blocker_kind.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let next_commands = gap_checks
        .iter()
        .flat_map(|check| check.next_commands.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    Some(ReadinessGap {
        id: spec.id.to_string(),
        status: status.to_string(),
        detail: format!("{}; unresolved checks:{unresolved}", spec.label),
        blocker_kinds,
        checks: gap_checks,
        next_commands,
    })
}

pub(super) fn render_gaps(out: &mut String, gaps: &[ReadinessGap]) {
    for gap in gaps {
        out.push_str(&format!(
            "gap\t{}\t{}\t{}\n",
            gap.id, gap.status, gap.detail
        ));
        if !gap.blocker_kinds.is_empty() {
            out.push_str(&format!(
                "gap_blocker_kinds\t{}\t{}\n",
                gap.id,
                gap.blocker_kinds.join(",")
            ));
        }
        for check in &gap.checks {
            out.push_str(&format!(
                "gap_check\t{}\t{}\t{}\t{}\n",
                gap.id, check.id, check.status, check.detail
            ));
            if let Some(kind) = &check.blocker_kind {
                out.push_str(&format!(
                    "gap_check_blocker_kind\t{}\t{}\t{}\n",
                    gap.id, check.id, kind
                ));
            }
            for (index, command) in check.next_commands.iter().enumerate() {
                out.push_str(&format!(
                    "gap_check_next\t{}\t{}\t{}\t{}\n",
                    gap.id,
                    check.id,
                    index + 1,
                    command
                ));
            }
        }
        for (index, command) in gap.next_commands.iter().enumerate() {
            out.push_str(&format!(
                "gap_next\t{}\t{}\t{}\n",
                gap.id,
                index + 1,
                command
            ));
        }
    }
}
