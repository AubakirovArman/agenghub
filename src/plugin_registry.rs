mod install;
mod lock;
#[cfg(test)]
mod tests;
mod types;

pub use install::{inspect_package, install_package, InstallOptions, InstallResult};
pub use lock::{list_installed, LockedPlugin, LockedSkill};
pub use types::{PluginManifest, PluginTrust};
