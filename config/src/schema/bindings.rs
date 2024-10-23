use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub(crate) struct BindingSchema {
    identifier: String,
    /// fixture.channel -> value
    actions: Vec<HashMap<String, String>>,
}

impl BindingSchema {
    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }

    pub fn get_actions(&self) -> &Vec<HashMap<String, String>> {
        &self.actions
    }
}
