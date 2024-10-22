use super::{utils::get_channel_addr, WebsocketHandlingError};
use crate::handlers::errors::ParseVariableError;
use artnet::ArtNetClient;
use config::Fixture;
use logger::log;
use regex::Regex;
use std::{collections::HashMap, sync::Arc};

pub fn handle(
    msg: &str,
    client: Arc<ArtNetClient>,
    fixtures: Arc<HashMap<String, Fixture>>,
) -> Result<(), WebsocketHandlingError> {
    // msg format needs to be "{fixture}.{channel}={value}"
    let capture_regex =
        Regex::new(r"^(?P<fixture>[^\.]+)\.(?P<channel>[^\.]+)=(?P<value>[0-9]+)$").unwrap();
    let captures = capture_regex
        .captures(msg)
        .ok_or_else(|| WebsocketHandlingError::InvalidActionOrDfcFormat(msg.to_string()))?;

    let fixture_res = captures.name("fixture");
    let channel_res = captures.name("channel");
    let value_res = captures.name("value");

    let fixture_name = match fixture_res {
        Some(fixture) => fixture.as_str(),
        None => {
            return Err(WebsocketHandlingError::InvalidActionOrDfcFormat(
                msg.to_string(),
            ));
        }
    };
    let channel_name = match channel_res {
        Some(channel) => channel.as_str(),
        None => {
            return Err(WebsocketHandlingError::InvalidActionOrDfcFormat(
                msg.to_string(),
            ));
        }
    };
    let value_str = match value_res {
        Some(value) => value.as_str(),
        None => {
            return Err(WebsocketHandlingError::InvalidActionOrDfcFormat(
                msg.to_string(),
            ));
        }
    };

    let value: u8 = value_str
        .parse()
        .map_err(|e| WebsocketHandlingError::VariableParseError(ParseVariableError::from(e)))?;

    let addr = get_channel_addr(&format!("{}.{}", fixture_name, channel_name), &fixtures);

    if addr.is_err() {
        return Err(WebsocketHandlingError::ChannelNotFound(
            channel_name.to_string(),
        ));
    }
    let addr = addr.unwrap();

    log!("Setting channel {} to value {}", addr, value);
    client.set_single(addr, value);

    // we don't need to commit the changes to the artnet nodes as this happens every n milliseconds
    Ok(())
}
