use super::defaults::*;
use super::types::{DiffLimitsSpec, TransactionSpec};

impl Default for TransactionSpec {
    fn default() -> Self {
        Self {
            approval_required: false,
            max_repair_attempts: default_max_repair_attempts(),
            rollback_on_failure: true,
            commit_on_success: true,
            memory_promotion: default_memory_promotion(),
            diff_limits: DiffLimitsSpec::default(),
        }
    }
}

impl Default for DiffLimitsSpec {
    fn default() -> Self {
        Self {
            max_files_changed: default_max_files_changed(),
            max_lines_added: default_max_lines_added(),
            max_lines_deleted: default_max_lines_deleted(),
        }
    }
}
