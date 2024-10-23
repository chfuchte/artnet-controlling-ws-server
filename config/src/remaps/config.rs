use crate::schema::config::*;

#[derive(Debug)]
pub struct Config {
    server: ServerConfigSchema,
    artnet: ArtNetConfigSchema,
}

impl Config {
    pub fn get_server_bind(&self) -> &str {
        &self.server.get_binds()
    }

    pub fn get_artnet_bind(&self) -> &str {
        &self.artnet.get_binds()
    }

    pub fn get_artnet_send(&self) -> &str {
        &self.artnet.get_sends()
    }

    pub fn get_artnet_universe(&self) -> u16 {
        self.artnet.get_universe()
    }

    pub fn get_artnet_broadcast(&self) -> bool {
        self.artnet.get_broadcast()
    }

    pub fn get_allow_direct_fixture_control(&self) -> bool {
        self.server.get_allow_direct_fixture_control()
    }

    /// default 50
    /// disabled <= 0
    pub fn get_send_every_ms(&self) -> u32 {
        self.server.get_send_artnet_every_ms().unwrap_or(50)
    }
}

pub fn map_config(config: &ConfigSchema) -> Config {
    Config {
        artnet: config.get_artnet_config().clone(),
        server: config.get_server_config().clone(),
    }
}
