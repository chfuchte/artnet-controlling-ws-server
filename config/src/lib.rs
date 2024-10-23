#[cfg(test)]
mod test;

mod errors;
mod remaps;
mod schema;

pub mod yaml;

pub use errors::ConfigParseError;
pub use remaps::binding::{Binding, KeyframesMode};
pub use remaps::config::Config;
pub use remaps::fixture::Fixture;
