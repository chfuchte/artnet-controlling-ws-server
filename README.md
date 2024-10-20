# Websocket-based ArtNet Controller

A websocket-based, file-configured lighting (Art-Net) controller, written in Rust.

## Table of Contents

-   [Installation](#installation)
    -   [Prebuilt Binary (Github Release)](#prebuilt-binary-github-release)
    -   [Building from Source](#building-from-source)
-   [Configuration](#configuration)
    -   [Server Configuration](#server-configuration)
    -   [ArtNet Configuration](#artnet-configuration)
    -   [Binding with Variables](#binding-with-variables)
-   [Frontend](#frontend)
-   [License](#license)

## Installation

### Prebuilt Binary (Github Release)

Download the latest release from the [GitHub release page](https://github.com/chfuchte/ws-artnet-controller/releases).  
Extract the archive and run the binary:

```bash
./server config.yaml
```

### Building from Source

**Prerequisites:**

-   [Git](https://git-scm.com/)
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
        binds: 0.0.0.0:3000
        allow_direct_fixture_control: true # (optional)
    artnet:
        binds: 0.0.0.0:6454
        sends: 255.255.255.255:6454
        broadcast: true
        universe: 0
        send_every_ms: 50 # (optional) (default: 50)
fixture_types:
    - name: any name you want to give to the fixture type
      channels:
          - name: list of channels for the fixture type (ordered by dmx channel address)
fixtures:
    - name: any name you want to give to the fixture
      type: name of the fixture type (needs to be present in fixture_types)
      start_addr: start address of the fixture in the universe (1-256)
bindings:
    - identifier: any unique identifier
      actions:
          - fixture.channel: value between 0 and 255 (channel is the channel name of the fixture type)
    - identifier: something{variable} # see below
      actions:
          - fixture.channel: "{variable}" # see below
```

### Server Configuration

#### `config.server.binds`

The ArtNet client binds to the given value. f.e. `0.0.0.0:3000`.

#### `config.server.allow_direct_fixture_control`

(Optional, default: false). If set to true, the server allows manipulating fixture channels without a binding. The server interprets any incoming message in the format `<fixture_name>.<channel>=<value>` as a direct fixture control message and applies it. The value must be an integer between 0 and 255.

It is not recommended to use this feature in a production environment or to use binding identifiers in a similar format to the direct fixture control messages. The following regex should not match any binding identifier: `^[^\.]+\.[^\.]+=[0-9]+$.`

### ArtNet Configuration

#### `config.artnet.binds`

The ArtNet client binds to the given value. f.e. `0.0.0.0:6454`.

#### `config.artnet.sends`

The server sends Art-Net messages to the given address and port. f.e. `127.0.0.1:6454` (localhost).

#### `config.artnet.broadcast`

If set to `true`, the server broadcasts the Art-Net messages to the network.

#### `config.artnet.universe`

The universe configured in the Art-Net node (which the node is listening to).

#### `config.artnet.send_every_ms`

Optional (default: 50). The server sends Art-Net messages every given milliseconds, regardless of the changes in the fixture channels.
This is necessary as [direct_fixture_control](#configserverallow_direct_fixture_control) is not sending the changes directly. If you don't use the direct fixture control feature, you may disable this feature by setting the value to `0`.

### Binding with Variables

You can use variables in the binding actions. The variables need to be enclosed in curly braces `{}`. The variables need to be present in the incoming websocket message. The server will replace the variables with the values from the incoming message.
The variables need to be positive integers between 0 and 255. This way you can set the fixture channels value dynamically from your frontend.

For example, you can use the following binding:

```yaml
bindings:
    - identifier: something{variable}
      actions:
          - fixture.channel: "{variable}"
```

The server will replace the `{variable}` with the value from the incoming message.
The message `something{200}` will set `fixture.channel` to `200`. (which would be the same as setting `fixture.channel` to `200` as the binding directly).

## Frontend

The [frontend](frontend/) is not hosted by the server as you may replace it with your own implementation.
The frontend is just a simple vite-based application compiling to a simple HTML file with some assets which you can open in your web browser or host it on your own.

## License

This project is licensed under the [MIT License](LICENSE.txt).
