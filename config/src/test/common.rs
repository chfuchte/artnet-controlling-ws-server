pub const YAML: &str = r#"
        config:
          server:
            binds: 0.0.0.0:3000
            allow_direct_fixture_control: true
            send_artnet_every_ms: 100
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
        - identifier: Binding2
          mode: once
          steps:
            - delay: 1000
              actions:
                - Fixture1.Channel1: 255
    "#;

pub const YAML_ONLY_NECCESSARY: &str = r#"
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
        - identifier: Binding2
          mode: once
          steps:
            - delay: 1000
              actions:
                - Fixture1.Channel1: 255
    "#;
