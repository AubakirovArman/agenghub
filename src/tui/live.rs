use std::io::{self, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;

use anyhow::Result;

use super::dashboard_text;

#[derive(Debug, Clone, Copy)]
pub struct LiveOptions {
    pub interval_ms: u64,
    pub once: bool,
}

pub fn live_dashboard(project_root: &Path, options: LiveOptions) -> Result<()> {
    loop {
        print!("\x1B[2J\x1B[H");
        println!("{}", dashboard_text(project_root)?);
        io::stdout().flush()?;
        if options.once {
            break;
        }
        thread::sleep(Duration::from_millis(options.interval_ms.max(250)));
    }
    Ok(())
}
