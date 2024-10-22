use super::{errors::WebsocketHandlingError, handle_default, handle_dfc, handle_vars};
use artnet::ArtNetClient;
use config::{Binding, Fixture};
use regex::Regex;
use std::{collections::HashMap, sync::Arc};

pub fn handle_websocket_message(
    msg: &str,
    client: Arc<ArtNetClient>,
    fixtures: Arc<HashMap<String, Fixture>>,
    bindings: Arc<HashMap<String, Binding>>,
    allow_direct_fixture_control: Arc<bool>,
) -> Result<(), WebsocketHandlingError> {
    if is_dfc(&msg) {
        if !*allow_direct_fixture_control {
            return Err(WebsocketHandlingError::DfcDisabledButMsgIsInDfcFormat);
        }
        return handle_dfc::handle(msg, client, fixtures);
    }

    if includes_variables(&msg) {
        return handle_vars::handle(msg, client, fixtures, bindings);
    }

    return handle_default::handle(msg, client, fixtures, bindings);
}

fn includes_variables(msg: &&str) -> bool {
    Regex::new(r".*?\{(.*?)\}.*?")
        .expect("failed to compile regex")
        .is_match(msg)
}

fn is_dfc(msg: &&str) -> bool {
    Regex::new(r"^[^\.]+\.[^\.]+=[0-9]+$")
        .expect("failed to compile regex")
        .is_match(msg)
}
