use crate::binding::{remap_bindings, Binding};
use crate::config::{map_config, Config};
use crate::fixture::{remap_fixtures, Fixture};
use crate::schema::Schema;
use serde_yaml::Error;
use std::collections::HashMap;

pub fn parse_yaml_into(
    yaml: &str,
) -> Result<(HashMap<String, Fixture>, HashMap<String, Binding>, Config), Error> {
    let config: Schema = serde_yaml::from_str(yaml).expect("Failed to parse YAML");

    let fixtures_map = remap_fixtures(config.fixtures, config.fixture_types).unwrap();
    let bindings_map: HashMap<String, Binding> =
        remap_bindings(config.bindings, &fixtures_map).unwrap();
    let config = map_config(config.config);

    Ok((fixtures_map, bindings_map, config))
}
