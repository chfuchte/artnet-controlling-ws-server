# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-10-22

The first release.

### Added

-   websocket server
-   threadsafe artnet client
-   logger crate for colored console logging
-   configuration file parsing (config crate)
-   message bindings with variables
-   direct fixture control

### Changed

-   N/A

### Deprecated

-   N/A

### Removed

-   N/A

### Fixed

-   N/A

### Security

-   N/A

## [0.2.0] - 2024-10-23

### Added

-   new method to define bindings: **steps** for sequential actions in a binding (full details in the README documentation)

### Changed

-   `send_artnet_every_ms` (known in version 0.1 as `send_every_ms`) is now moved to `config.server` instead of `config.artnet`

### Deprecated

-   N/A

### Removed

-   prebuilts for 32-bit systems

### Fixed

-   error handling

### Security

-   N/A
