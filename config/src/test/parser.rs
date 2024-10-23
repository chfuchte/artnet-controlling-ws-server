use super::common::{YAML, YAML_ONLY_NECCESSARY};
use crate::{remaps::binding::StepsMode, yaml::parse_yaml, Binding};

#[test]
fn test_parse_fixtures() {
    let result = parse_yaml(&YAML);
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
    let result = parse_yaml(&YAML);
    assert!(result.is_ok());

    let (_, bindings_map, _) = result.unwrap();
    assert!(bindings_map.get("Binding1").is_some());
    let binding = bindings_map.get("Binding1").unwrap();
    match binding {
        Binding::WithActions(bnd) => {
            assert_eq!(bnd.get_identifier(), "Binding1")
        }
        _ => {
            panic!("Binding2 is not a WithActions binding")
        }
    }
}

#[test]
fn test_parse_steps() {
    let result = parse_yaml(&YAML);
    assert!(result.is_ok());

    let (_, bindings_map, _) = result.unwrap();
    assert!(bindings_map.get("Binding2").is_some());
    let binding = bindings_map.get("Binding2").unwrap();
    match binding {
        Binding::WithSteps(bnd) => {
            assert_eq!(bnd.get_identifier(), "Binding2");
            match bnd.get_mode() {
                StepsMode::Once => {
                    assert!(true)
                }
                _ => {
                    panic!("Binding2 is not a Once binding")
                }
            }
        }
        _ => {
            panic!("Binding2 is not a WithActions binding")
        }
    }
}

#[test]
fn test_parse_config() {
    let result = parse_yaml(&YAML);
    assert!(result.is_ok());

    let (_, _, config) = result.unwrap();
    assert_eq!(config.get_server_bind(), "0.0.0.0:3000");
    assert_eq!(config.get_artnet_bind(), "0.0.0.0:6454");
    assert_eq!(config.get_artnet_send(), "255.255.255.255:6454");
    assert_eq!(config.get_artnet_universe(), 0);
    assert_eq!(config.get_allow_direct_fixture_control(), true);
    assert_eq!(config.get_send_every_ms(), 100)
}

#[test]
fn test_parse_config_only_necessary() {
    let result = parse_yaml(&YAML_ONLY_NECCESSARY);
    assert!(result.is_ok());

    let (_, _, config) = result.unwrap();
    assert_eq!(config.get_allow_direct_fixture_control(), false);
    assert_eq!(config.get_send_every_ms(), 50);
}
