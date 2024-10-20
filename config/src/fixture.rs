use crate::schema::SchemaFixture;

use super::schema::SchemaFixtureType;
use std::{collections::HashMap, fmt::Error};

#[derive(Debug)]
pub struct Fixture {
    #[allow(dead_code)]
    identifier: String,
    channels: HashMap<String, u16>,
}

impl Fixture {
    pub(super) fn new(identifier: &str, start_addr: u16, fixture_type: &SchemaFixtureType) -> Fixture {
        let mut channels: HashMap<String, u16> = HashMap::new();
        for (i, channel) in fixture_type.channels.iter().enumerate() {
            channels.insert(channel.name.clone(), start_addr + i as u16);
        }
        Fixture {
            identifier: identifier.to_string(),
            channels,
        }
    }

    #[allow(dead_code)]
    // used in wshandlers::mod.rs
    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }

    pub fn get_channel_addr(&self, channel_name: &str) -> Option<&u16> {
        self.channels.get(channel_name)
    }
}

pub fn remap_fixtures(
    fixtures: Vec<SchemaFixture>,
    fixture_types: Vec<SchemaFixtureType>,
) -> Result<HashMap<String, Fixture>, Error> {
    let mut fixture_types_map: HashMap<String, SchemaFixtureType> = HashMap::new();

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

    Ok(fixtures_map)
}
