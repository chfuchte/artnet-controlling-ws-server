use super::schema::YAMLFixtureType;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Fixture {
    #[allow(dead_code)]
    identifier: String,
    channels: HashMap<String, u16>,
}

impl Fixture {
    pub(super) fn new(identifier: &str, start_addr: u16, fixture_type: &YAMLFixtureType) -> Fixture {
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
