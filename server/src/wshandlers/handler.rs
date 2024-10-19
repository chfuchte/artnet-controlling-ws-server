use crate::config::{Binding, Fixture};
use artnet::ArtNetClient;
use super::parse_variable::extract_variables;
use regex::Regex;
use std::{collections::HashMap, sync::Arc};

pub enum WebsocketHandlingError {
    IoError(std::io::Error),
    UnknownMessage(String),
}

pub fn handle_websocket_message(
    msg: &str,
    client: Arc<ArtNetClient>,
    fixtures: Arc<HashMap<String, Fixture>>,
    bindings: Arc<HashMap<String, Binding>>,
) -> Result<(), WebsocketHandlingError> {
    let does_msg_include_variables_regex: Regex = Regex::new(r".*?{(.*?)}.*?").unwrap();
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
    let replace_regex = Regex::new(r"{.*?}").unwrap();
    let find_result = bindings.iter().find(|(identifier, _)| {
        // check if msg and identifier are the same if we remove the variables
        // replace with {replaced}
        let replaced_identifier = replace_regex
            .replace_all(identifier, "{replaced}")
            .to_string();
        let replaced_msg = replace_regex.replace_all(msg, "{replaced}").to_string();
        replaced_identifier == replaced_msg
    });
    let binding = find_result.map(|(_, binding)| binding);
    if binding.is_none() {
        return Err(WebsocketHandlingError::UnknownMessage(format!(
            "Unknown message: {}",
            msg
        )));
    }
    let binding = binding.unwrap();
    let variables = extract_variables(msg, binding.get_identifier());

    binding.get_actions().iter().for_each(|action| {
        let channel_addr = get_channel_addr(&action[0], fixtures.clone());
        let value: u8 = action[1]
            .chars()
            .fold(action[1].to_string(), |acc, c| {
                if c == '{' {
                    let variable_name = acc
                        .chars()
                        .skip_while(|c| *c != '{')
                        .skip(1)
                        .take_while(|c| *c != '}')
                        .collect::<String>();
                    let variable_value = variables
                        .get(&variable_name)
                        .expect(&format!("Variable {} not found", variable_name));
                    acc.replace(&format!("{{{}}}", variable_name), &variable_value.to_string())
                } else {
                    acc
                }
            })
            .parse()
            .unwrap();
        client.set_single(channel_addr, value);
    });

    client.commit().map_err(WebsocketHandlingError::IoError)
}

fn handle_message_without_variables(
    msg: &str,
    client: Arc<ArtNetClient>,
    fixtures: Arc<HashMap<String, Fixture>>,
    bindings: Arc<HashMap<String, Binding>>,
) -> Result<(), WebsocketHandlingError> {
    let binding = bindings.get(msg);
    if binding.is_none() {
        return Err(WebsocketHandlingError::UnknownMessage(format!(
            "Unknown message: {}",
            msg
        )));
    }

    binding.unwrap().get_actions().iter().for_each(|action| {
        let channel_addr = get_channel_addr(&action[0], fixtures.clone());
        let value: u8 = action[1].parse().unwrap();
        client.set_single(channel_addr, value);
    });

    client.commit().map_err(WebsocketHandlingError::IoError)
}

fn get_channel_addr(
    action_fixture_dot_channnel_str: &str,
    fixtures: Arc<HashMap<String, Fixture>>,
) -> u16 {
    let splited_action_fixture_dot_channnel_str: Vec<&str> =
        action_fixture_dot_channnel_str.split('.').collect();
    *fixtures
        .get(splited_action_fixture_dot_channnel_str[0])
        .expect(&format!(
            "Fixture {} not found",
            splited_action_fixture_dot_channnel_str[0]
        ))
        .get_channel_addr(splited_action_fixture_dot_channnel_str[1])
        .expect(&format!(
            "Channel {} not found",
            splited_action_fixture_dot_channnel_str[1]
        ))
}
