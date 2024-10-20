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
}

pub fn map_config(config: SchemaConfig) -> Config {
    Config {
        artnet: config.artnet,
        server: config.server,
    }
}
