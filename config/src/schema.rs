use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub(crate) struct SchemaChannel {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct SchemaFixtureType {
    pub name: String,
    pub channels: Vec<SchemaChannel>,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub(crate) struct SchemaConfig {
    pub server: SchemaConfigServer,
    pub artnet: SchemaConfigArtNet,
}

#[derive(Deserialize, Debug)]
pub(crate) struct SchemaConfigServer {
    pub binds: String,
    pub allow_direct_fixture_control: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct SchemaConfigArtNet {
    pub binds: String,
    pub sends: String,
    pub universe: u16,
    pub broadcast: bool,
    pub send_every_ms: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Schema {
    pub fixture_types: Vec<SchemaFixtureType>,
    pub fixtures: Vec<SchemaFixture>,
    pub bindings: Vec<SchemaBinding>,
    pub config: SchemaConfig,
}
