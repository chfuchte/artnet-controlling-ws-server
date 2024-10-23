use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct FixtureSchema {
    name: String,
    start_addr: u16,
    #[serde(rename = "type")]
    fixture_type_name: String,
}

impl FixtureSchema {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_start_addr(&self) -> &u16 {
        &self.start_addr
    }

    pub fn get_fixture_type_name(&self) -> &str {
        &self.fixture_type_name
    }
}
