use super::{
    utils::{extract_variables, get_channel_addr, substitute_variable},
    WebsocketHandlingError,
};
use artnet::ArtNetClient;
use config::{Binding, Fixture};
use regex::Regex;
use std::{collections::HashMap, sync::Arc};

pub fn handle(
    msg: &str,
    client: Arc<ArtNetClient>,
    fixtures: Arc<HashMap<String, Fixture>>,
    bindings: Arc<HashMap<String, Binding>>,
) -> Result<(), WebsocketHandlingError> {
    let replace_regex = Regex::new(r"\{.*?\}").unwrap();

    let binding = bindings
        .iter()
        .find(|(identifier, _)| {
            let replaced_identifier = replace_regex.replace_all(identifier, "{replaced}");
            let replaced_msg = replace_regex.replace_all(msg, "{replaced}");
            replaced_identifier == replaced_msg
        })
        .map(|(_, binding)| binding)
        .ok_or_else(|| WebsocketHandlingError::UnknownMessage(msg.to_string()))?;

    let variables: HashMap<String, u8> = extract_variables(msg, binding.get_identifier())
        .map_err(|e| WebsocketHandlingError::VariableParseError(e))?;

    for action in binding.get_actions() {
        let channel_addr = get_channel_addr(&action[0], &fixtures)?;
        let value = substitute_variable(&action[1], &variables)?;
        client.set_single(channel_addr, value);
    }

    // Commit all changes to the ArtNetClient
    client.commit().map_err(WebsocketHandlingError::IoError)
}
