use chrono::Utc;
use uuid::Uuid;

pub(super) fn new_tx_id() -> String {
    let suffix = Uuid::new_v4().to_string();
    format!("tx-{}-{}", Utc::now().format("%Y%m%d%H%M%S"), &suffix[..8])
}
