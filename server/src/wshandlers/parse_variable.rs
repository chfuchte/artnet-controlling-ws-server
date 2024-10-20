use std::{collections::HashMap, fmt::Error};

/// Extracts variables from a message.
/// msg = "something{255}something_else"
/// identifier = "something{variable}something_else"
/// returns { "variable": 255 }
/// may also return an empty hashmap if no variables are found
/// may also return multiple variables if multiple are found
pub fn extract_variables(msg: &str, identifier: &str) -> Result<HashMap<String, u8>, Error> {
    let mut result: HashMap<String, u8> = HashMap::new();
    let mut var_iter = identifier.split('{').skip(1).map(|s| s.split('}').next().unwrap());
    let mut val_iter = msg.split('{').skip(1).map(|s| s.split('}').next().unwrap());

    while let (Some(variable), Some(value)) = (var_iter.next(), val_iter.next()) {
        let parsed_value = value.parse::<u8>().expect(format!("Failed to parse {} as u8", value).as_str());
        result.insert(variable.to_string(), parsed_value);
    }

    Ok(result)
}
