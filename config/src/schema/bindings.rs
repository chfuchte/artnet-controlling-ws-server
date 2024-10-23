use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub(crate) struct BindingSchema {
    identifier: String,
    /// fixture.channel -> value
    actions: Option<Vec<HashMap<String, String>>>,
    /// once, alternate, random, repeat
    mode: Option<String>,
    steps: Option<Vec<BindingKeyframesSteps>>,
}

impl BindingSchema {
    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }

    pub fn get_actions(&self) -> &Option<Vec<HashMap<String, String>>> {
        &self.actions
    }

    /// once, alternate, random, repeat
    pub fn get_mode(&self) -> &Option<String> {
        &self.mode
    }

    pub fn get_steps(&self) -> &Option<Vec<BindingKeyframesSteps>> {
        &self.steps
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct BindingKeyframesSteps {
    /// delay between last and this step (ms)
    delay: u64,
    actions: Vec<HashMap<String, String>>,
}

impl BindingKeyframesSteps {
    pub fn get_delay(&self) -> u64 {
        self.delay
    }

    pub fn get_actions(&self) -> &Vec<HashMap<String, String>> {
        &self.actions
    }
}
