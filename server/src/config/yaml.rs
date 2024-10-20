use crate::config::binding::Binding;
use crate::config::fixture::Fixture;
use crate::config::schema::{YAMLBinding, YAMLConfig, YAMLFixture, YAMLFixtureType};
use serde_yaml::Error;
use std::collections::HashMap;

pub fn parse_yaml_into(
    yaml: &str,
) -> Result<(HashMap<String, Fixture>, HashMap<String, Binding>), Error> {
    let config: YAMLConfig = serde_yaml::from_str(yaml).expect("Failed to parse YAML");

    let fixtures_map = remap_fixtures(config.fixtures, config.fixture_types);
    let bindings_map: HashMap<String, Binding> = remap_bindings(config.bindings);

    Ok((fixtures_map, bindings_map))
}

fn remap_fixtures(
    fixtures: Vec<YAMLFixture>,
    fixture_types: Vec<YAMLFixtureType>,
) -> HashMap<String, Fixture> {
    let mut fixture_types_map: HashMap<String, YAMLFixtureType> = HashMap::new();
    for fixture_type in fixture_types {
        fixture_types_map.insert(fixture_type.name.clone(), fixture_type);
    }
    let mut fixtures_map: HashMap<String, Fixture> = HashMap::new();
    for fixture in fixtures {
        fixtures_map.insert(
            fixture.name.clone(),
            Fixture::new(
                &fixture.name,
                fixture.start_addr,
                &fixture_types_map
                    .get(&fixture.fixture_type)
                    .expect("fixture type not found"),
            ),
        );
    }
    fixtures_map
}

fn remap_bindings(bindings: Vec<YAMLBinding>) -> HashMap<String, Binding> {
    let mut bindings_map: HashMap<String, Binding> = HashMap::new();
    for binding in bindings {
        let mut actions: Vec<[String; 2]> = Vec::new();
        for action in binding.actions {
            actions.push([
                action.keys().next().unwrap().clone(),
                action.values().next().unwrap().clone(),
            ]);
        }
        bindings_map.insert(
            binding.identifier.clone(),
            Binding::new(&binding.identifier, actions),
        );
    }
    bindings_map
}
