use std::collections::HashMap;

/// Extracts variables from a message.
/// msg = "something{255}something_else"
/// identifier = "something{variable}something_else"
/// returns { "variable": 255 }
/// may also return an empty hashmap if no variables are found
/// may also return multiple variables if multiple are found
pub fn extract_variables(msg: &str, identifier: &str) -> HashMap<String, u8> {
    let mut result: HashMap<String, u8> = HashMap::new();
    let mut var_iter = identifier.split('{').skip(1).map(|s| s.split('}').next().unwrap());
    let mut val_iter = msg.split('{').skip(1).map(|s| s.split('}').next().unwrap());

    while let (Some(variable), Some(value)) = (var_iter.next(), val_iter.next()) {
        if let Ok(parsed_value) = value.parse::<u8>() {
            result.insert(variable.to_string(), parsed_value);
        }
    }

    result
}
