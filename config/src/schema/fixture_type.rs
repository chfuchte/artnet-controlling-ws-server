use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct FixtureTypeSchema {
    name: String,
    channels: Vec<FixtureTypeChannelSchema>,
}

impl FixtureTypeSchema {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_channels(&self) -> &[FixtureTypeChannelSchema] {
        &self.channels
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct FixtureTypeChannelSchema {
    name: String,
}

impl FixtureTypeChannelSchema {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
