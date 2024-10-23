use crate::{
    schema::fixture::FixtureSchema, schema::fixture_type::FixtureTypeSchema, ConfigParseError,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Fixture {
    #[allow(dead_code)]
    identifier: String,
    channels: HashMap<String, u16>,
}

impl Fixture {
    pub(super) fn new(
        identifier: &str,
        start_addr: u16,
        fixture_type: &FixtureTypeSchema,
    ) -> Fixture {
        let mut channels: HashMap<String, u16> = HashMap::new();
        for (i, channel) in fixture_type.get_channels().iter().enumerate() {
            channels.insert(channel.get_name().to_string(), start_addr + i as u16);
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
    fixtures: &[FixtureSchema],
    fixture_types: &[FixtureTypeSchema],
) -> Result<HashMap<String, Fixture>, ConfigParseError> {
    let mut fixture_types_map: HashMap<String, FixtureTypeSchema> = HashMap::new();

    for fixture_type in fixture_types {
        fixture_types_map.insert(fixture_type.get_name().to_string(), fixture_type.clone());
    }

    let mut fixtures_map: HashMap<String, Fixture> = HashMap::new();

    for fixture in fixtures {
        let fixture_type = fixture_types_map.get(fixture.get_fixture_type_name());

        if fixture_type.is_none() {
            return Err(ConfigParseError::FixtureTypeNotFound(
                fixture.get_fixture_type_name().to_string(),
                fixture.get_name().to_string(),
            ));
        }

        fixtures_map.insert(
            fixture.get_name().to_string(),
            Fixture::new(
                &fixture.get_name(),
                *fixture.get_start_addr(),
                &fixture_type.unwrap(),
            ),
        );
    }

    Ok(fixtures_map)
}
