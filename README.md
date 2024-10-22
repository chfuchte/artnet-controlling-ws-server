# Extentible Art-Net controlling WebSocket Server

A configuration file-based WebSocket server providing lighting control via the Art-Net protocol, allowing you to build your preferred frontend without caring about the backend.

## Table of Contents

-   [Features](#features)
-   [Installation](#installation)
    -   [Prebuilt Binary (Github Release)](#prebuilt-binary-github-release)
    -   [Building from Source](#building-from-source)
-   [Usage](#usage)
    -   [Configuration File Schema](#configuration-file-schema)
    -   [Send Data regularly](#send-data-regularly)
    -   [Direct Fixture Control](#direct-fixture-control)
    -   [Bindings](#bindings)
    -   [Bindings with Variables](#bindings-with-variables)
-   [Development](#development)
-   [License](#license)

## Features

**WebSockets**: Control up to 512 channels via a single WebSocket connection.
**Configurable**: Everything is configured in a YAML configuration file to make it easy to set up.
**Direct fixture control**: Allows direct control of fixture channels without requiring predefined bindings.
**Variable support in bindings**: Use variables to make them more flexible.

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

## Usage

Everything is configured in a single configuration file. The configuration file needs to be in the YAML format and needs to be passed as an argument to the server binary. It will be parsed and loaded at server startup.

```bash
./server /path/to/config.yaml
```

### Configuration File Schema

```yaml
config:
    server:
        binds: # string (required)
        allow_direct_fixture_control: # boolean (optional) (default: false)
    artnet:
        binds: # string (required)
        sends: # string (required)
        broadcast: # boolean (required)
        universe: # integer (required) (1-32768)
        send_every_ms: # integer (optional) (default: 50)
fixture_types:
    - name: # string (required)
      channels:
          - name: # string (required)
    # as many fixture types as you want
fixtures:
    - name: # string (required)
      type: # string (required) (name of the fixture type)
      start_addr: # integer (required) (1-256)
    # as many fixtures as you want
bindings:
    - identifier: # string (required)
      actions:
          - fixture.channel: # integer (required) (0-255)
    # as many bindings as you want
```

| Key                                          | Type    | Required | Default | Description                                                      |
| -------------------------------------------- | ------- | -------- | ------- | ---------------------------------------------------------------- |
| `config.server.binds`                        | string  | yes      | -       | The TCP server binds to the given value. f.e. `0.0.0.0:3000`.    |
| `config.server.allow_direct_fixture_control` | boolean | no       | false   | [Direct Fixture Control](#direct-fixture-control)                |
| `config.artnet.binds`                        | string  | yes      | -       | The ArtNet client binds to the given value. f.e. `0.0.0.0:6454`. |
| `config.artnet.sends`                        | string  | yes      | -       | address to send Art-Net packages to f.e. `0.0.0.0:6454`          |
| `config.artnet.broadcast`                    | boolean | yes      | -       | wheather to broadcast the Art-Net packages or not                |
| `config.artnet.universe`                     | integer | yes      | -       | the universe configured in the Art-Net node                      |
| `config.artnet.send_every_ms`                | integer | no       | 50      | [Send Data regularly](#send-data-regularly)                      |

### Send Data regularly

The server sends the Art-Net packages every n milliseconds, regardless of the changes. This can be useful keep the chances of package loss low.
The default is every 50 milliseconds and can be changed at `config.artnet.send_every_ms`. You can disable this feature by setting the value to `0`.

> [!IMPORTANT]
> If you want to use [direct_fixture_control](#direct-fixture-control) you are not able to disable this feature as direct fixture control does not send every single channel change directly (other as bindings do).

### Direct Fixture Control

Needs to be specified in the configuration file as `config.server.allow_direct_fixture_control` (optional, default: false).
If set to true, the server allows manipulating fixture channels without a binding. The server interprets any incoming message in the format `<fixture_name>.<channel>=<value>` as a direct fixture control message and applies it. The value must be an integer between 0 and 255.

> [!IMPORTANT]
> It is not possiple to use binding identifiers in a similar format to the direct fixture control messages even if it is turned off.
> The following regex should not match any binding identifier: `^[^\.]+\.[^\.]+=[0-9]+$.`

### Bindings

Bindings are used to map incoming websocket messages to fixture channels. The bindings need to be specified in the configuration file as a list of bindings. Each binding has a unique identifier and a list of actions. The actions are executed in order when the binding is triggered.
The format of the action is `<fixture_name>.<channel>: <value>`. The value must be an integer between 0 and 255. The server will set the fixture channel to the given value.

### Bindings with Variables

If you want to don't rely on hardcoded values, you can use variables in your bindings. The variables need to be enclosed in curly braces `{}` in the `identifier` and the `message`. The variables need to be present in the incoming websocket message. The server will replace the variables with the values from the incoming message.
The variable values of an incomming message need to be positive integers between 0 and 255.

The following binding would set the `fixture.channel` to `200` if the incoming message is `hello{200}`. This would have the same effect as setting `fixture.channel` to `200` by hand in the configuration file.

```yaml
bindings:
    - identifier: hello{variable}
      actions:
          - fixture.channel: "{variable}"
```

You may also use multiple variables in a single binding. The following binding would set the `fixture.channel` to `200` and the `fixture2.channel` to `100` if the incoming message is `hello{200}world{100}`.

```yaml
bindings:
    - identifier: hello{variable1}world{variable2}
      actions:
          - fixture.channel: "{variable1}"
          - fixture2.channel: "{variable2}"
```

> [!NOTE]
> Every variable needs to be unique in a single binding although it may be used multiple times in the list of actions.

## Development

### Prerequisites

-   [Git](https://git-scm.com/)
-   [Rust Programming Language](https://www.rust-lang.org/tools/install)

### Project Structure

The project is split into multiple crates:

-   `artnet`: library providing the Art-Net client
-   `config`: library providing the configuration parsing
-   `logger`: library providing a simple colored console logger
-   `server`: the main binary running the server and handling the websocket connections

### Running the Server

You may want to use the [`config.yaml`](examples/dev/config.yaml) configuration file for testing and playing around on your local machine.
The `config.yaml` should have all configurations possible set and at least one example per feature.

```bash
cargo debug
# or
cargo run -- examples/dev/config.yaml`
```

In case you need a debug client, you can use the [`client.html`](examples/dev/client.html) which sets up a websocket connection to `ws://localhost:3000`.

### Testing

Each crate has its own unit tests. You can run the tests with the following command either in the root directory or in the specific crate directory:

```bash
cargo test
```

## License

This project is licensed under the [MIT License](LICENSE.txt).
