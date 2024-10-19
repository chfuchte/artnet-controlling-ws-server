use artnet::ArtNetClient;
use config::Fixture;
use std::{collections::HashMap, sync::Arc};

pub enum WebsocketHandlingError {
    IoError(std::io::Error),
    UnknownMessage(String),
}

pub fn handle_websocket_message(
    msg: &str,
    client: Arc<ArtNetClient>,
    _fixtures: Arc<HashMap<String, Fixture>>,
) -> Result<(), WebsocketHandlingError> {
    match msg {
        "all::dimmer::full" => {
            client.set_single(0, 255);
            client.set_single(10, 255);
            client.commit().map_err(WebsocketHandlingError::IoError)
        }
        "all::dimmer::off" => {
            client.set_single(0, 0);
            client.set_single(10, 0);
            client.commit().map_err(WebsocketHandlingError::IoError)
        }
        _ => Err(WebsocketHandlingError::UnknownMessage(format!(
            "Unknown message: {}",
            msg
        ))),
    }
}
