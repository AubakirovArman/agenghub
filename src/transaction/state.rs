use std::time::Duration;

use crate::adaptive::AdaptiveDecision;
use crate::command_runner::{RemoteRunner, RunnerMetadata};
use crate::diff_guard::DiffGuardResult;
use crate::observability::CostProfile;
use crate::reviewer::ReviewResult;
use crate::smart_sync::SmartSyncDecision;
use crate::verifier::{VerifierIntegrationArtifact, VerifierResult};
use crate::workspace::{PreparedWorkspace, WorkspaceRuntimeMetadata};

use super::TransactionStatus;

#[derive(Default)]
pub(super) struct RunState {
    pub(super) prepared: Option<PreparedWorkspace>,
    pub(super) diff_guard: Option<DiffGuardResult>,
    pub(super) review: Option<ReviewResult>,
    pub(super) verifier: Option<VerifierResult>,
    pub(super) verifier_integration: Option<VerifierIntegrationArtifact>,
    pub(super) sync: Option<SmartSyncDecision>,
    pub(super) workspace_runtime: Option<WorkspaceRuntimeMetadata>,
    pub(super) runner: Option<RunnerMetadata>,
    pub(super) cost_profile: Option<CostProfile>,
    pub(super) adaptive: Option<AdaptiveDecision>,
    pub(super) error_fingerprint: Option<String>,
    pub(super) failure_reason: Option<String>,
    pub(super) remote_runner: Option<RemoteRunner>,
    pub(super) command_timeout_secs: u64,
    pub(super) committed: bool,
    pub(super) status: Option<TransactionStatus>,
}

impl RunState {
    pub(super) fn command_timeout(&self) -> Duration {
        let secs = if self.command_timeout_secs == 0 {
            300
        } else {
            self.command_timeout_secs
        };
        Duration::from_secs(secs)
    }
}
