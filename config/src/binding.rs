use crate::schema::SchemaBinding;
use std::collections::HashMap;

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

    pub fn get_actions(&self) -> &Vec<[String; 2]> {
        &self.actions
    }
}

pub fn remap_bindings(bindings: Vec<SchemaBinding>) -> HashMap<String, Binding> {
    let mut bindings_map: HashMap<String, Binding> = HashMap::new();
    for binding in bindings {
        let mut actions: Vec<[String; 2]> = Vec::new();
        for action in binding.actions {
            actions.push([
                action.keys().next().unwrap().clone(),
                action.values().next().unwrap().clone(),
            ]);
        }
        bindings_map.insert(
            binding.identifier.clone(),
            Binding::new(&binding.identifier, actions),
        );
    }
    bindings_map
}
