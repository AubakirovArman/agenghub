pub mod bootstrap;
pub mod config;
pub mod doctor;
pub mod ecosystem;
mod env;
pub mod open;
pub mod providers;
pub mod readiness;

#[cfg(test)]
mod tests;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
