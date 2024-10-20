use serde_yaml::Error;
use std::collections::HashMap;
use crate::config::schema::{YAMLConfig, YAMLFixtureType};
use crate::config::fixture::Fixture;
use crate::config::binding::Binding;

pub fn parse_yaml_into(
    yaml: &str,
) -> Result<(HashMap<String, Fixture>, HashMap<String, Binding>), Error> {
    let config: YAMLConfig = serde_yaml::from_str(yaml).expect("Failed to parse YAML");

    let mut fixture_types_map: HashMap<String, YAMLFixtureType> = HashMap::new();
    for fixture_type in config.fixture_types {
        fixture_types_map.insert(fixture_type.name.clone(), fixture_type);
    }

    let mut fixtures_map: HashMap<String, Fixture> = HashMap::new();
    for fixture in config.fixtures {
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

    let mut bindings_map: HashMap<String, Binding> = HashMap::new();
    for binding in config.bindings {
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

    Ok((fixtures_map, bindings_map))
}
