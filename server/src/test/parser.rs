use crate::config::parse_yaml_into;

#[test]
fn test_parse_yaml_into() {
    let yaml = r#"
        fixture_types:
        - name: FixtureType1
          channels:
          - name: Channel1
        fixtures:
        - name: Fixture1
          type: FixtureType1
          start_addr: 1
    "#;

    let result = parse_yaml_into(&yaml);
    assert!(result.is_ok());
    let map = result.unwrap();
    assert!(map.get("Fixture1").is_some());
    assert!(map.get("Fixture1").unwrap().get_identifier() == "Fixture1");
    assert!(map.get("Fixture1").unwrap().get_channel_addr("Channel1") == 1);
}
