use crate::schema::{SchemaConfig, SchemaConfigArtNet, SchemaConfigServer};

#[derive(Debug)]
pub struct Config {
    server: SchemaConfigServer,
    artnet: SchemaConfigArtNet,
}

impl Config {
    pub fn get_server_bind(&self) -> &str {
        &self.server.binds
    }

    pub fn get_artnet_bind(&self) -> &str {
        &self.artnet.binds
    }

    pub fn get_artnet_send(&self) -> &str {
        &self.artnet.sends
    }

    pub fn get_artnet_universe(&self) -> u16 {
        self.artnet.universe
    }

    pub fn get_artnet_broadcast(&self) -> bool {
        self.artnet.broadcast
    }

    pub fn get_allow_direct_fixture_control(&self) -> bool {
        self.server.allow_direct_fixture_control.unwrap_or(false)
    }

    pub fn get_send_every_ms(&self) -> Option<u64> {
        self.artnet.send_every_ms
    }
}

pub fn map_config(config: SchemaConfig) -> Config {
    Config {
        artnet: config.artnet,
        server: config.server,
    }
}
