#[cfg(test)]
mod test;

mod parser;
pub use parser::{Fixture, parse_yaml_into};
