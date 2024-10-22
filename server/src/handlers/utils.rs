use super::WebsocketHandlingError;
use config::Fixture;
use regex::Regex;
use std::{collections::HashMap, num::ParseIntError, sync::Arc};

pub fn get_channel_addr(
    fixture_name: &str,
    channel_name: &str,
    fixtures: &Arc<HashMap<String, Fixture>>,
) -> Result<u16, WebsocketHandlingError> {
    let fixture = fixtures
        .get(fixture_name)
        .ok_or_else(|| WebsocketHandlingError::FixtureNotFound(fixture_name.to_string()))?;

    let channel_addr = fixture.get_channel_addr(channel_name).ok_or_else(|| {
        WebsocketHandlingError::ChannelNotFound(fixture_name.to_string(), channel_name.to_string())
    })?;

    Ok(*channel_addr)
}

/// Extracts variables from a message.
/// msg = "something{255}something_else"
/// identifier = "something{variable}something_else"
/// returns { "variable": 255 }
/// may also return an empty hashmap if no variables are found
/// may also return multiple variables if multiple are found
pub fn extract_variables(
    msg: &str,
    identifier: &str,
) -> Result<HashMap<String, u8>, ParseIntError> {
    let mut result: HashMap<String, u8> = HashMap::new();
    let mut var_iter = identifier
        .split('{')
        .skip(1)
        .map(|s| s.split('}').next().unwrap());
    let mut val_iter = msg.split('{').skip(1).map(|s| s.split('}').next().unwrap());

    while let (Some(variable), Some(value)) = (var_iter.next(), val_iter.next()) {
        let parsed_value = value.parse::<u8>()?;
        result.insert(variable.to_string(), parsed_value);
    }

    Ok(result)
}

/// replaces a variable in a string with its value
pub fn substitute_variable(
    value_str: &str,
    variables: &HashMap<String, u8>,
) -> Result<u8, WebsocketHandlingError> {
    let replace_regex = Regex::new(r"\{.*?\}").unwrap();

    if let Some(caps) = replace_regex.captures(value_str) {
        let variable = &caps[0][1..caps[0].len() - 1]; // Strip the braces { }

        variables
            .get(variable)
            .copied()
            .ok_or_else(|| WebsocketHandlingError::ExtractVariableError(value_str.to_string()))
    } else {
        Err(WebsocketHandlingError::ExtractVariableError(
            value_str.to_string(),
        ))
    }
}
