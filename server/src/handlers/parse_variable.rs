use std::collections::HashMap;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct ParseVariableError {
    details: String,
}
impl ParseVariableError {
    fn new(msg: &str) -> ParseVariableError {
        ParseVariableError {
            details: msg.to_string(),
        }
    }
}
impl fmt::Display for ParseVariableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl From<ParseIntError> for ParseVariableError {
    fn from(_: ParseIntError) -> Self {
        ParseVariableError::new("Failed to parse variable value as u8")
    }
}

/// Extracts variables from a message.
/// msg = "something{255}something_else"
/// identifier = "something{variable}something_else"
/// returns { "variable": 255 }
/// may also return an empty hashmap if no variables are found
/// may also return multiple variables if multiple are found
pub fn extract_variables(
    msg: &str,
    identifier: &str,
) -> Result<HashMap<String, u8>, ParseVariableError> {
    let mut result: HashMap<String, u8> = HashMap::new();
    let mut var_iter = identifier
        .split('{')
        .skip(1)
        .map(|s| s.split('}').next().unwrap());
    let mut val_iter = msg.split('{').skip(1).map(|s| s.split('}').next().unwrap());

    while let (Some(variable), Some(value)) = (var_iter.next(), val_iter.next()) {
        let parsed_value = value.parse::<u8>().map_err(|_| {
            ParseVariableError::new(&format!("failed to parse variable to u8 in msg {}", &msg))
        })?;
        result.insert(variable.to_string(), parsed_value);
    }

    Ok(result)
}
