# Config Parser Crate

Crate to parse the yaml configuration file and check if the required fields are present.

## Current State

This crate does not parse any .yaml file and is specifically designed to parse the configuration file for this project.

## YAML Schema

The schema for the configuration file is as follows:

```yaml
fixture_types:
    - name: any name you want to give to the fixture type
      channels:
          - name: list of channels for the fixture type (ordered by dmx channel address)
fixtures:
    - name: any name you want to give to the fixture
      type: name of the fixture type (needs to be present in fixture_types)
      start_addr: 1
```
