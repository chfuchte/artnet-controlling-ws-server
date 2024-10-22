use crate::handlers::utils::extract_variables;

#[test]
fn test_extract_variable() {
    let msg = "something{255}something_else";
    let identifier = "something{variable}something_else";
    let result = extract_variables(msg, identifier);
    let mut expected = std::collections::HashMap::new();
    expected.insert("variable".to_string(), 255);
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_extract_variable_multiple() {
    let msg = "something{255}something_else{42}another_thing";
    let identifier = "something{variable}something_else{variable2}another_thing";
    let result = extract_variables(msg, identifier);
    let mut expected = std::collections::HashMap::new();
    expected.insert("variable".to_string(), 255);
    expected.insert("variable2".to_string(), 42);
    assert_eq!(result.unwrap(), expected);
}
