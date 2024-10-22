use crate::{schema::SchemaBinding, ConfigParseError, Fixture};
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
pub struct Binding {
    identifier: String,
    actions: Vec<[String; 2]>,
}

impl Binding {
    pub(super) fn new(identifier: &str, actions: Vec<[String; 2]>) -> Binding {
        Binding {
            identifier: identifier.to_string(),
            actions,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }

    /// [key, value]
    pub fn get_actions(&self) -> &Vec<[String; 2]> {
        &self.actions
    }
}

pub fn remap_bindings(
    bindings: Vec<SchemaBinding>,
    fixtures: &HashMap<String, Fixture>,
) -> Result<HashMap<String, Binding>, ConfigParseError> {
    let mut bindings_map: HashMap<String, Binding> = HashMap::new();

    for binding in bindings {
        let mut actions: Vec<[String; 2]> = Vec::new();

        for action in binding.actions {
            let key = action.keys().next().unwrap().clone();
            let value = action.values().next().unwrap().clone();

            let pair_exists = fixture_channel_exists(&key, fixtures);
            if !pair_exists {
                return Err(ConfigParseError::BindingFixtureChannelDoesNotExist(key));
            }

            actions.push([key, value]);
        }

        bindings_map.insert(
            binding.identifier.clone(),
            Binding::new(&binding.identifier, actions),
        );
    }

    Ok(bindings_map)
}

fn fixture_channel_exists(
    action_fixture_dot_channnel_str: &str,
    fixtures: &HashMap<String, Fixture>,
) -> bool {
    let splited_action_fixture_dot_channnel_str: Vec<&str> =
        action_fixture_dot_channnel_str.split('.').collect();

    let fixture = fixtures.get(splited_action_fixture_dot_channnel_str[0]);

    if fixture.is_none() {
        return false;
    }

    fixture
        .unwrap()
        .get_channel_addr(splited_action_fixture_dot_channnel_str[1])
        .is_some()
}
