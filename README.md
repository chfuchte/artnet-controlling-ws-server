# Websocket-based ArtNet Controller 

A websocket-based, file-configured lighting (Art-Net™) controller, written in Rust.

## Table of Contents
- [Installation](#installation)
- [Configuration](#configuration)
- [License](#license)

## Installation

### Prebuilt Binary (Github Release)

Download the latest release from the current release.
Extract the archive and run the binary as follows:

```bash
./server config.yaml
```

### Building from Source

**Prerequisites:**

- [Rust Programming Language](https://www.rust-lang.org/tools/install)

```bash
git clone https://github.com/chfuchte/ws-artnet-controller.git # or download the source code from the latest release
cd ws-artnet-controller
cargo build --release
./target/release/server config.yaml
```

## Configuration

The configuration file needs to be in the YAML format and needs to be passed as an argument to the server binary.
You can find some configuration examples in the [examples directory](examples/).
Configuration file schema:

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

## License

This project is licensed under the [MIT License](LICENSE.txt).
