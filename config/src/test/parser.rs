use crate::parse_yaml_into;

const YAML: &str = r#"
    fixture_types:
        - name: FixtureType1
          channels:
          - name: Channel1
        fixtures:
        - name: Fixture1
          type: FixtureType1
          start_addr: 1
        bindings:
        - identifier: Binding1
          actions:
          - Channel1: 1
    "#;

#[test]
fn test_parse_fixtures() {
    let result = parse_yaml_into(&YAML);
    assert!(result.is_ok());

    let (fixture_map, _) = result.unwrap();
    assert!(fixture_map.get("Fixture1").is_some());
    assert_eq!(
        fixture_map.get("Fixture1").unwrap().get_identifier(),
        "Fixture1"
    );
    assert_eq!(
        fixture_map
            .get("Fixture1")
            .unwrap()
            .get_channel_addr("Channel1")
            .unwrap(),
        &1
    );
}

#[test]
fn test_parse_bindings() {
    let result = parse_yaml_into(&YAML);
    assert!(result.is_ok());

    let (_, bindings_map) = result.unwrap();
    assert!(bindings_map.get("Binding1").is_some());
    assert_eq!(
        bindings_map.get("Binding1").unwrap().get_identifier(),
        "Binding1"
    );

    let actions = bindings_map.get("Binding1").unwrap().get_actions();
    assert_eq!(actions[0][0], "Channel1");
    assert_eq!(actions[0][1], "1");
}
