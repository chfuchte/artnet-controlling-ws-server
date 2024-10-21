use std::{collections::HashMap, sync::Arc};
use config::Fixture;
use super::WebsocketHandlingError;

pub fn get_channel_addr(
    action_fixture_dot_channel_str: &str,
    fixtures: Arc<HashMap<String, Fixture>>,
) -> Result<u16, WebsocketHandlingError> {
    let split: Vec<&str> = action_fixture_dot_channel_str.split('.').collect();
    if split.len() != 2 {
        return Err(WebsocketHandlingError::InvalidActionFormat(
            action_fixture_dot_channel_str.to_string(),
        ));
    }

    let fixture_name = split[0];
    let channel_name = split[1];

    let fixture = fixtures
        .get(fixture_name)
        .ok_or_else(|| WebsocketHandlingError::FixtureNotFound(fixture_name.to_string()))?;

    let channel_addr = fixture
        .get_channel_addr(channel_name)
        .ok_or_else(|| WebsocketHandlingError::ChannelNotFound(channel_name.to_string()))?;

    Ok(*channel_addr)
}
