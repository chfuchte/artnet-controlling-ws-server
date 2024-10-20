use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub(crate) struct SchemaChannel {
    pub name: String,
}

#[derive(Deserialize)]
pub(crate) struct SchemaFixtureType {
    pub name: String,
    pub channels: Vec<SchemaChannel>,
}

#[derive(Deserialize)]
pub(crate) struct SchemaFixture {
    pub name: String,
    pub start_addr: u16,
    #[serde(rename = "type")]
    pub fixture_type: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct SchemaBinding {
    pub identifier: String,
    /// fixture.channel -> value
    pub actions: Vec<HashMap<String, String>>,
}

#[derive(Deserialize)]
pub(crate) struct SchemaConfig {
    pub fixture_types: Vec<SchemaFixtureType>,
    pub fixtures: Vec<SchemaFixture>,
    pub bindings: Vec<SchemaBinding>,
}
