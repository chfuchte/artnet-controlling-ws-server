use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ConfigSchema {
    server: ServerConfigSchema,
    artnet: ArtNetConfigSchema,
}

impl ConfigSchema {
    pub fn get_server_config(&self) -> &ServerConfigSchema {
        &self.server
    }

    pub fn get_artnet_config(&self) -> &ArtNetConfigSchema {
        &self.artnet
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ServerConfigSchema {
    binds: String,
    allow_direct_fixture_control: Option<bool>,
    send_artnet_every_ms: Option<u32>,
}

impl ServerConfigSchema {
    pub fn get_binds(&self) -> &str {
        &self.binds
    }

    pub fn get_allow_direct_fixture_control(&self) -> bool {
        self.allow_direct_fixture_control.unwrap_or(false)
    }

    pub fn get_send_artnet_every_ms(&self) -> &Option<u32> {
        &self.send_artnet_every_ms
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ArtNetConfigSchema {
    binds: String,
    sends: String,
    universe: u16,
    broadcast: bool,
}

impl ArtNetConfigSchema {
    pub fn get_binds(&self) -> &str {
        &self.binds
    }

    pub fn get_sends(&self) -> &str {
        &self.sends
    }

    pub fn get_universe(&self) -> u16 {
        self.universe
    }

    pub fn get_broadcast(&self) -> bool {
        self.broadcast
    }
}
