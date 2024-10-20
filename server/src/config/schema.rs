use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct YAMLChannel {
    pub name: String,
}

#[derive(Deserialize)]
pub(super) struct YAMLFixtureType {
    pub name: String,
    pub channels: Vec<YAMLChannel>,
}

#[derive(Deserialize)]
pub(super) struct YAMLFixture {
    pub name: String,
    pub start_addr: u16,
    #[serde(rename = "type")]
    pub fixture_type: String,
}

#[derive(Deserialize, Debug)]
pub(super) struct YAMLBinding {
    pub identifier: String,
    /// fixture.channel -> value
    pub actions: Vec<HashMap<String, String>>,
}

#[derive(Deserialize)]
pub(super) struct YAMLConfig {
    pub fixture_types: Vec<YAMLFixtureType>,
    pub fixtures: Vec<YAMLFixture>,
    pub bindings: Vec<YAMLBinding>,
}
