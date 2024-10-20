use crate::parse_yaml_into;

const YAML: &str = r#"
        config:
          server:
            binds: 0.0.0.0:3000
            allow_direct_fixture_control: true
          artnet:
            binds: 0.0.0.0:6454
            sends: 255.255.255.255:6454
            broadcast: true
            universe: 0
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
          - Fixture1.Channel1: 1
    "#;

const YAML_ONLY_NECCESSARY: &str = r#"
        config:
          server:
            binds: 0.0.0.0:3000
          artnet:
            binds: 0.0.0.0:6454
            sends: 255.255.255.255:6454
            broadcast: true
            universe: 0
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
          - Fixture1.Channel1: 1
    "#;

#[test]
fn test_parse_fixtures() {
    let result = parse_yaml_into(&YAML);
    assert!(result.is_ok());

    let (fixture_map, _, _) = result.unwrap();
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

    let (_, bindings_map, _) = result.unwrap();
    assert!(bindings_map.get("Binding1").is_some());
    assert_eq!(
        bindings_map.get("Binding1").unwrap().get_identifier(),
        "Binding1"
    );

    let actions = bindings_map.get("Binding1").unwrap().get_actions();
    assert_eq!(actions[0][0], "Fixture1.Channel1");
    assert_eq!(actions[0][1], "1");
}

#[test]
fn test_parse_config() {
    let result = parse_yaml_into(&YAML);
    assert!(result.is_ok());

    let (_, _, config) = result.unwrap();
    assert_eq!(config.get_server_bind(), "0.0.0.0:3000");
    assert_eq!(config.get_artnet_bind(), "0.0.0.0:6454");
    assert_eq!(config.get_artnet_send(), "255.255.255.255:6454");
    assert_eq!(config.get_artnet_universe(), 0);
    assert_eq!(config.get_allow_direct_fixture_control(), true);
}

#[test]
fn test_parse_config_only_necessary() {
    let result = parse_yaml_into(&YAML_ONLY_NECCESSARY);
    assert!(result.is_ok());

    let (_, _, config) = result.unwrap();
    assert_eq!(config.get_allow_direct_fixture_control(), false);
}
