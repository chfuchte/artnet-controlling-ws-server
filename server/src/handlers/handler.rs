use super::{
    utils::get_channel_addr, with_dfc::handle_message_with_direct_fixture_contol, with_vars::handle_message_with_variables
};
use artnet::ArtNetClient;
use config::{Binding, Fixture};
use logger::log;
use regex::Regex;
use std::{collections::HashMap, sync::Arc};

#[derive(Debug)]
pub enum WebsocketHandlingError {
    IoError(std::io::Error),
    UnknownMessage(String),
    InvalidActionFormat(String),
    FixtureNotFound(String),
    ChannelNotFound(String),
    VariableNotFound(String),
    ParseError(String),
}

impl From<std::io::Error> for WebsocketHandlingError {
    fn from(err: std::io::Error) -> Self {
        WebsocketHandlingError::IoError(err)
    }
}

pub fn handle_websocket_message(
    msg: &str,
    client: Arc<ArtNetClient>,
    fixtures: Arc<HashMap<String, Fixture>>,
    bindings: Arc<HashMap<String, Binding>>,
    allow_direct_fixture_control: Arc<bool>,
) -> Result<(), WebsocketHandlingError> {
    let does_msg_include_variables_regex = Regex::new(r".*?\{(.*?)\}.*?").unwrap();
    let is_direct_fixture_control_regex = Regex::new(r"^[^\.]+\.[^\.]+=[0-9]+$").unwrap();
    if is_direct_fixture_control_regex.is_match(msg) && *allow_direct_fixture_control {
        handle_message_with_direct_fixture_contol(msg, client, fixtures)
    } else if does_msg_include_variables_regex.is_match(msg) {
        handle_message_with_variables(msg, client, fixtures, bindings)
    } else {
        handle_message_without_variables(msg, client, fixtures, bindings)
    }
}

fn handle_message_without_variables(
    msg: &str,
    client: Arc<ArtNetClient>,
    fixtures: Arc<HashMap<String, Fixture>>,
    bindings: Arc<HashMap<String, Binding>>,
) -> Result<(), WebsocketHandlingError> {
    // find the binding that matches the message
    let binding = bindings
        .get(msg)
        .ok_or_else(|| WebsocketHandlingError::UnknownMessage(msg.to_string()))?;

    // set the channels to the values
    for action in binding.get_actions() {
        let channel_addr = get_channel_addr(&action[0], fixtures.clone())?;
        let value: u8 = action[1].parse().map_err(|_| {
            WebsocketHandlingError::ParseError(format!(
                "Failed to parse value into u8: {}",
                action[1]
            ))
        })?;

        log!("Setting channel {} to {}", channel_addr, value);
        client.set_single(channel_addr, value);
    }

    // commit the changes to the artnet nodes via the client
    client.commit().map_err(WebsocketHandlingError::IoError)
}
