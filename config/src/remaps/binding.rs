use crate::{schema::bindings::BindingSchema, ConfigParseError, Fixture};
use std::{collections::HashMap, fmt::Debug};

pub enum Binding {
    WithActions(ActionBinding),
    WithKeyframes(KeyframeBinding),
}

impl Binding {
    pub fn get_identifier(&self) -> &str {
        match self {
            Binding::WithActions(action_binding) => action_binding.get_identifier(),
            Binding::WithKeyframes(keyframe_binding) => keyframe_binding.get_identifier(),
        }
    }
}

#[derive(Debug)]
pub struct ActionBinding {
    identifier: String,
    actions: Vec<[String; 2]>,
}

impl ActionBinding {
    pub(super) fn new(identifier: &str, actions: Vec<[String; 2]>) -> ActionBinding {
        ActionBinding {
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

#[derive(Debug)]
pub struct KeyframeBinding {
    identifier: String,
    mode: KeyframesMode,
    steps: Vec<KeyframeStep>,
}

impl KeyframeBinding {
    pub(super) fn new(
        identifier: &str,
        mode: KeyframesMode,
        steps: Vec<KeyframeStep>,
    ) -> KeyframeBinding {
        KeyframeBinding {
            identifier: identifier.to_string(),
            mode,
            steps,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }

    pub fn get_mode(&self) -> &KeyframesMode {
        &self.mode
    }

    /// [key, value]
    pub fn get_steps(&self) -> &Vec<KeyframeStep> {
        &self.steps
    }
}

#[derive(Debug)]
pub struct KeyframeStep {
    delay: u64,
    actions: Vec<[String; 2]>,
}

impl KeyframeStep {
    pub fn get_delay(&self) -> u64 {
        self.delay
    }

    /// [key, value]
    pub fn get_actions(&self) -> &Vec<[String; 2]> {
        &self.actions
    }
}

#[derive(Debug)]
pub enum KeyframesMode {
    Alernate,
    Once,
}

pub fn remap_bindings(
    bindings: &[BindingSchema],
    fixtures: &HashMap<String, Fixture>,
) -> Result<HashMap<String, Binding>, ConfigParseError> {
    let mut bindings_map: HashMap<String, Binding> = HashMap::new();

    for binding in bindings {
        if binding.get_mode().is_some() && binding.get_steps().is_some() {
            // keyframes mode
            let mode = match binding.get_mode().as_ref().unwrap().as_str() {
                "alternate" => KeyframesMode::Alernate,
                "once" => KeyframesMode::Once,
                mode => return Err(ConfigParseError::InvalidKeyframesMode(mode.to_string())),
            };

            let mut steps: Vec<KeyframeStep> = Vec::new();

            for step in binding.get_steps().as_ref().unwrap() {
                let mut actions: Vec<[String; 2]> = Vec::new();

                for action in step.get_actions() {
                    let key = action.keys().next().unwrap().clone();
                    let value = action.values().next().unwrap().clone();

                    let pair_exists = fixture_channel_exists(&key, fixtures);
                    if !pair_exists {
                        return Err(ConfigParseError::BindingFixtureChannelDoesNotExist(key));
                    }

                    actions.push([key, value]);
                }

                steps.push(KeyframeStep {
                    delay: step.get_delay(),
                    actions,
                });
            }

            bindings_map.insert(
                binding.get_identifier().to_string(),
                Binding::WithKeyframes(KeyframeBinding::new(
                    &binding.get_identifier(),
                    mode,
                    steps,
                )),
            );
        } else if binding.get_actions().is_some()
            && binding.get_mode().is_none()
            && binding.get_steps().is_none()
        {
            // actions mode
            let mut actions: Vec<[String; 2]> = Vec::new();

            for action in binding.get_actions().clone().unwrap() {
                let key = action.keys().next().unwrap().clone();
                let value = action.values().next().unwrap().clone();

                let pair_exists = fixture_channel_exists(&key, fixtures);
                if !pair_exists {
                    return Err(ConfigParseError::BindingFixtureChannelDoesNotExist(key));
                }

                actions.push([key, value]);
            }

            bindings_map.insert(
                binding.get_identifier().to_string(),
                Binding::WithActions(ActionBinding::new(&binding.get_identifier(), actions)),
            );
        } else {
            return Err(ConfigParseError::InvalidActionOrKeyframesBinding);
        }
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
