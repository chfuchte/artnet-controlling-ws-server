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

#[derive(Deserialize)]
struct YAMLConfig {
    fixture_types: Vec<YAMLFixtureType>,
    fixtures: Vec<YAMLFixture>,
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

pub fn parse_yaml_into(yaml: &str) -> Result<HashMap<String, Fixture>, Error> {
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

    Ok(fixtures_map)
}
