use std::{collections::HashMap, sync::Arc};
use logger::log;
use regex::Regex;
use artnet::ArtNetClient;
use config::Fixture;

use super::{utils::get_channel_addr, WebsocketHandlingError};

pub fn handle_message_with_direct_fixture_contol(
    msg: &str,
    client: Arc<ArtNetClient>,
    fixtures: Arc<HashMap<String, Fixture>>,
) -> Result<(), WebsocketHandlingError> {
    // msg format needs to be "{fixture}.{channel}={value}"
    let re =
        Regex::new(r"^(?P<fixture>[^\.]+)\.(?P<channel>[^\.]+)=(?P<value>[0-9]+)$").unwrap();
    let cap = re.captures(msg).ok_or_else(|| {
        WebsocketHandlingError::UnknownMessage(format!("No binding found for message: {}", msg))
    })?;
    let fixture_name = cap.name("fixture").unwrap().as_str();
    let channel_name = cap.name("channel").unwrap().as_str();
    let value: u8 = cap.name("value").unwrap().as_str().parse().map_err(|_| {
        WebsocketHandlingError::ParseError(format!(
            "Failed to parse value into u8: {}",
            cap.name("value").unwrap().as_str()
        ))
    })?;
    let addr = get_channel_addr(&format!("{}.{}", fixture_name, channel_name), fixtures);
    if addr.is_err() {
        return Err(WebsocketHandlingError::ChannelNotFound(
            channel_name.to_string(),
        ));
    }
    let addr = addr.unwrap();
    log!("Setting channel {} to {}", addr, value);
    client.set_single(addr, value);
    // we don't need to commit the changes to the artnet nodes as this happens every n milliseconds
    Ok(())
}