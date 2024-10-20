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
