use crate::binding::{remap_bindings, Binding};
use crate::fixture::{remap_fixtures, Fixture};
use crate::schema::SchemaConfig;
use serde_yaml::Error;
use std::collections::HashMap;

pub fn parse_yaml_into(
    yaml: &str,
) -> Result<(HashMap<String, Fixture>, HashMap<String, Binding>), Error> {
    let config: SchemaConfig = serde_yaml::from_str(yaml).expect("Failed to parse YAML");

    let fixtures_map = remap_fixtures(config.fixtures, config.fixture_types);
    let bindings_map: HashMap<String, Binding> = remap_bindings(config.bindings);

    Ok((fixtures_map, bindings_map))
}
