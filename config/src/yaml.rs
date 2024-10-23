use crate::remaps::binding::remap_bindings;
use crate::remaps::config::map_config;
use crate::remaps::fixture::remap_fixtures;
use crate::schema::Schema;
use crate::{Binding, Config, ConfigParseError, Fixture};
use std::collections::HashMap;

pub fn parse_yaml(
    yaml: &str,
) -> Result<(HashMap<String, Fixture>, HashMap<String, Binding>, Config), ConfigParseError> {
    let parsed_configuration: Schema =
        serde_yaml::from_str(yaml).map_err(ConfigParseError::YamlError)?;

    let parsed_fixtures = parsed_configuration.get_fixtures();
    let parsed_fixture_types = parsed_configuration.get_fixture_types();
    let parsed_bindings = parsed_configuration.get_bindings();
    let parsed_config = parsed_configuration.get_config();

    let fixtures_map = remap_fixtures(parsed_fixtures, parsed_fixture_types)?;
    let bindings_map: HashMap<String, Binding> = remap_bindings(parsed_bindings, &fixtures_map)?;
    let config = map_config(&parsed_config);

    Ok((fixtures_map, bindings_map, config))
}
