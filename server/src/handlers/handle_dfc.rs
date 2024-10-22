use super::{utils::get_channel_addr, WebsocketHandlingError};
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
        .expect(&format!("Regex should match message {}", msg));

    let fixture_res = captures.name("fixture");
    let channel_res = captures.name("channel");
    let value_res = captures.name("value");

    let fixture_name = fixture_res
        .expect("fixture_name should be present")
        .as_str();
    let channel_name = channel_res
        .expect("channel_name should be present")
        .as_str();
    let value_str = value_res.expect("value should be present").as_str();

    let value: u8 = value_str
        .parse()
        .map_err(|e| WebsocketHandlingError::ParseVariableToNumError(e))?;

    let addr = get_channel_addr(fixture_name, channel_name, &fixtures)?;

    log!(
        "Setting {} (dmx {}) to value {}",
        &format!("{}.{}", fixture_name, channel_name),
        addr,
        value
    );
    client.set_single(addr, value);

    // we don't need to commit the changes to the artnet nodes as this happens every n milliseconds
    Ok(())
}
