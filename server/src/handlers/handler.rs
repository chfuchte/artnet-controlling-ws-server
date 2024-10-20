use super::parse_variable::extract_variables;
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
) -> Result<(), WebsocketHandlingError> {
    let does_msg_include_variables_regex = Regex::new(r".*?\{(.*?)\}.*?").unwrap();
    
    if does_msg_include_variables_regex.is_match(msg) {
        handle_message_with_variables(msg, client, fixtures, bindings)
    } else {
        handle_message_without_variables(msg, client, fixtures, bindings)
    }
}

fn handle_message_with_variables(
    msg: &str,
    client: Arc<ArtNetClient>,
    fixtures: Arc<HashMap<String, Fixture>>,
    bindings: Arc<HashMap<String, Binding>>,
) -> Result<(), WebsocketHandlingError> {
    let replace_regex = Regex::new(r"\{.*?\}").unwrap();
    
    // find the binding that matches the message
    let binding = bindings.iter().find(|(identifier, _)| {
        let replaced_identifier = replace_regex.replace_all(identifier, "{replaced}");
        let replaced_msg = replace_regex.replace_all(msg, "{replaced}");
        replaced_identifier == replaced_msg
    }).map(|(_, binding)| binding);
    
    let binding = binding.ok_or_else(|| WebsocketHandlingError::UnknownMessage(
        format!("No binding found for message: {}", msg)
    ))?;
    
    // extract variables from the message
    let variables = extract_variables(msg, binding.get_identifier())
        .map_err(|e| WebsocketHandlingError::ParseError(
            format!("Failed to extract variables from message: {}", e)
        ))?;
    
    // replace variables and set the channels to the values
    for action in binding.get_actions() {
        let channel_addr = get_channel_addr(&action[0], fixtures.clone())?;
        
        let value_str = substitute_variables(&action[1], &variables)?;
        let value: u8 = value_str.parse().map_err(|_| WebsocketHandlingError::ParseError(format!("Failed to parse value into u8: {}", value_str)))?;
        
        log(&format!("Setting channel {} to {}", channel_addr, value));
        client.set_single(channel_addr, value);
    }

    // commit the changes to the artnet nodes via the client
    client.commit().map_err(WebsocketHandlingError::IoError)
}

fn handle_message_without_variables(
    msg: &str,
    client: Arc<ArtNetClient>,
    fixtures: Arc<HashMap<String, Fixture>>,
    bindings: Arc<HashMap<String, Binding>>,
) -> Result<(), WebsocketHandlingError> {
    // find the binding that matches the message
    let binding = bindings.get(msg).ok_or_else(|| WebsocketHandlingError::UnknownMessage(msg.to_string()))?;
    
    // set the channels to the values
    for action in binding.get_actions() {
        let channel_addr = get_channel_addr(&action[0], fixtures.clone())?;
        let value: u8 = action[1].parse().map_err(|_| WebsocketHandlingError::ParseError(format!("Failed to parse value into u8: {}", action[1])))?;
        
        log(&format!("Setting channel {} to {}", channel_addr, value));
        client.set_single(channel_addr, value);
    }

    // commit the changes to the artnet nodes via the client
    client.commit().map_err(WebsocketHandlingError::IoError)
}

fn get_channel_addr(
    action_fixture_dot_channel_str: &str,
    fixtures: Arc<HashMap<String, Fixture>>,
) -> Result<u16, WebsocketHandlingError> {
    let split: Vec<&str> = action_fixture_dot_channel_str.split('.').collect();
    if split.len() != 2 {
        return Err(WebsocketHandlingError::InvalidActionFormat(action_fixture_dot_channel_str.to_string()));
    }

    let fixture_name = split[0];
    let channel_name = split[1];

    let fixture = fixtures.get(fixture_name)
        .ok_or_else(|| WebsocketHandlingError::FixtureNotFound(fixture_name.to_string()))?;

    let channel_addr = fixture.get_channel_addr(channel_name)
        .ok_or_else(|| WebsocketHandlingError::ChannelNotFound(channel_name.to_string()))?;
    
    Ok(*channel_addr)
}

fn substitute_variables(value: &str, variables: &HashMap<String, u8>) -> Result<String, WebsocketHandlingError> {
    let mut result = value.to_string();

    let variable_regex = Regex::new(r"\{(.*?)\}").unwrap();
    for caps in variable_regex.captures_iter(value) {
        let var_name = &caps[1];
        let var_value = variables.get(var_name)
            .ok_or_else(|| WebsocketHandlingError::VariableNotFound(var_name.to_string()))?;
        result = result.replace(&format!("{{{}}}", var_name), var_value.to_string().as_str());
    }

    Ok(result)
}
