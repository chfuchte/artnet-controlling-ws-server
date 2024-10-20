# Websocket-based ArtNet Controller

A websocket-based, file-configured lighting (Art-Net™) controller, written in Rust.

## Table of Contents

-   [Installation](#installation)
    -   [Prebuilt Binary (Github Release)](#prebuilt-binary-github-release)
    -   [Building from Source](#building-from-source)
-   [Configuration](#configuration)
-   [Frontend](#frontend)
-   [License](#license)

## Installation

### Prebuilt Binary (Github Release)

Download the latest release from the current release.
Extract the archive and run the binary as follows:

```bash
./server config.yaml
```

### Building from Source

**Prerequisites:**

-   [Rust Programming Language](https://www.rust-lang.org/tools/install)

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
config:
    server:
        binds: 0.0.0.0:3000 # where should the websocket server listen for incoming connections
        allow_direct_fixture_control: true # (optional) whether all messages in the format of "fixture.channel=value" should be allowed
    artnet:
        binds: 0.0.0.0:6454
        sends: 255.255.255.255:6454 # where should the artnet data be sent to
        broadcast: true # whether to broadcast the artnet data or not
        universe: 0 # universe
fixture_types:
    - name: any name you want to give to the fixture type
      channels:
          - name: list of channels for the fixture type (ordered by dmx channel address)
fixtures:
    - name: any name you want to give to the fixture
      type: name of the fixture type (needs to be present in fixture_types)
      start_addr: 1
bindings:
    - identifier: any unique identifier
      actions:
          - fixture.channel: 255 # any value (u8) between 0 and 255
    - identifier: something{variable} # something{255}, something{0}, ... would be valid as messages from a client
      actions:
          - fixture.channel: "{variable}"
```

## Frontend

The [frontend](frontend/) is not hosted by the server as you may replace it with your own implementation.
The frontend is just a simple vite-based application compiling to a simple HTML file with some assets which you can open in your web browser or host it on your own.

## License

This project is licensed under the [MIT License](LICENSE.txt).
