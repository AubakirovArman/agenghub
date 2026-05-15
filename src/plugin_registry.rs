mod install;
mod lock;
mod scaffold;
#[cfg(test)]
mod tests;
mod types;

pub use install::{inspect_package, install_package, InstallOptions, InstallResult};
pub use lock::{list_installed, LockedPlugin, LockedSkill};
pub use scaffold::{scaffold_package, ScaffoldOptions};
pub use types::{PluginManifest, PluginTrust};
