use serde::Deserialize;
use serde_yaml::Error;
use std::collections::HashMap;

#[derive(Deserialize)]
struct YAMLChannel {
    name: String,
}

#[derive(Deserialize)]
struct YAMLFixtureType {
    name: String,
    channels: Vec<YAMLChannel>,
}

#[derive(Deserialize)]
struct YAMLFixture {
    name: String,
    start_addr: u16,
    #[serde(rename = "type")]
    fixture_type: String,
}

#[derive(Deserialize, Debug)]
struct YAMLBinding {
    identifier: String,
    /// fixture.channel -> value
    actions: Vec<HashMap<String, String>>,
}

#[derive(Deserialize)]
struct YAMLConfig {
    fixture_types: Vec<YAMLFixtureType>,
    fixtures: Vec<YAMLFixture>,
    bindings: Vec<YAMLBinding>,
}

#[derive(Debug)]
pub struct Fixture {
    identifier: String,
    channels: HashMap<String, u16>,
}

impl Fixture {
    pub(self) fn new(identifier: &str, start_addr: u16, fixture_type: &YAMLFixtureType) -> Fixture {
        let mut channels: HashMap<String, u16> = HashMap::new();
        for (i, channel) in fixture_type.channels.iter().enumerate() {
            channels.insert(channel.name.clone(), start_addr + i as u16);
        }
        Fixture {
            identifier: identifier.to_string(),
            channels,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }

    pub fn get_channel_addr(&self, channel_name: &str) -> u16 {
        *self.channels.get(channel_name).unwrap()
    }

    pub fn get_channels(&self) -> &HashMap<String, u16> {
        &self.channels
    }
}

pub struct Binding {
    identifier: String,
    actions: Vec<[String; 2]>,
}

impl Binding {
    pub fn new(identifier: &str, actions: Vec<[String; 2]>) -> Binding {
        Binding {
            identifier: identifier.to_string(),
            actions,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }

    pub fn get_actions(&self) -> &Vec<[String; 2]> {
        &self.actions
    }
}

pub fn parse_yaml_into(
    yaml: &str,
) -> Result<(HashMap<String, Fixture>, HashMap<String, Binding>), Error> {
    let config: YAMLConfig = serde_yaml::from_str(yaml).expect("Failed to parse YAML");

    dbg!(&config.bindings);

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
