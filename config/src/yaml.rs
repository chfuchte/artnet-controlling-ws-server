use crate::binding::{remap_bindings, Binding};
use crate::config::{map_config, Config};
use crate::fixture::{remap_fixtures, Fixture};
use crate::schema::Schema;
use crate::ConfigParseError;
use std::collections::HashMap;

pub fn parse_yaml_into(
    yaml: &str,
) -> Result<(HashMap<String, Fixture>, HashMap<String, Binding>, Config), ConfigParseError> {
    let config: Schema = serde_yaml::from_str(yaml).map_err(ConfigParseError::YamlError)?;

    let fixtures_map = remap_fixtures(config.fixtures, config.fixture_types)?;
    let bindings_map: HashMap<String, Binding> = remap_bindings(config.bindings, &fixtures_map)?;
    let config = map_config(config.config);

    Ok((fixtures_map, bindings_map, config))
}
