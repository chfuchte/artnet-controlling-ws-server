use bindings::BindingSchema;
use config::ConfigSchema;
use fixture::FixtureSchema;
use fixture_type::FixtureTypeSchema;
use serde::Deserialize;

pub(crate) mod bindings;
pub(crate) mod config;
pub(crate) mod fixture;
pub(crate) mod fixture_type;

#[derive(Deserialize, Debug)]
pub(crate) struct Schema {
    fixture_types: Vec<FixtureTypeSchema>,
    fixtures: Vec<FixtureSchema>,
    bindings: Vec<BindingSchema>,
    config: ConfigSchema,
}

impl Schema {
    pub fn get_fixture_types(&self) -> &[FixtureTypeSchema] {
        &self.fixture_types
    }

    pub fn get_fixtures(&self) -> &[FixtureSchema] {
        &self.fixtures
    }

    pub fn get_bindings(&self) -> &[BindingSchema] {
        &self.bindings
    }

    pub fn get_config(&self) -> &ConfigSchema {
        &self.config
    }
}
