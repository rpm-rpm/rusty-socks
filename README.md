# rusty-socks
SOCKS Proxy in Rust

# SOCKS5 Proxy Server

A lightweight, asynchronous SOCKS5 proxy server implementation in Rust using Tokio.

## Features

- **SOCKS5 Protocol Support**: Full implementation of the SOCKS5 protocol (RFC 1928)
- **Authentication**: Supports no-authentication method
- **Address Types**: IPv4, IPv6, and domain name resolution
- **CONNECT Command**: Supports TCP connection establishment
- **Asynchronous I/O**: Built with Tokio for high performance
- **Optimized Binary**: Release builds are optimized for minimal size

## Requirements

- Rust 1.70 or later
- Cargo package manager

## Installation

1. Clone or download the project
2. Navigate to the project directory
3. Build the project:

```bash
cargo build --release
```

The optimized binary will be available at `target/release/socks_proxy.exe` (Windows) or `target/release/socks_proxy` (Linux/macOS).

## Usage

### Running the Server

Start the proxy server:

```bash
cargo run --release
```

Or run the compiled binary:

```bash
./target/release/socks_proxy
```

The server will start listening on `127.0.0.1:1080` (the standard SOCKS5 port).

### Configuration

Currently, the server binds to `127.0.0.1:1080` by default. To modify the bind address, edit the `addr` variable in `main.rs`.

### Client Configuration

Configure your applications to use the SOCKS5 proxy:

- **Address**: `127.0.0.1`
- **Port**: `1080`
- **Authentication**: None (no username/password required)

#### Example Applications:

**Firefox**:
1. Go to Settings → Network Settings
2. Select "Manual proxy configuration"
3. Enter `127.0.0.1` in SOCKS Host and `1080` in Port
4. Select SOCKS v5

**curl**:
```bash
curl --socks5 127.0.0.1:1080 https://example.com
```

**Chrome/Chromium** (via command line):
```bash
chromium --proxy-server=socks5://127.0.0.1:1080
```

## Protocol Implementation

The server implements the following SOCKS5 features:

### Handshake Phase
1. Receives client greeting with supported authentication methods
2. Responds with chosen authentication method (currently only no-auth)

### Request Phase
1. Receives connection request with:
   - Command (CONNECT only)
   - Destination address (IPv4, IPv6, or domain name)
   - Destination port

### Connection Phase
1. Establishes connection to destination server
2. Sends success/failure response to client
3. Forwards data bidirectionally between client and destination

## Architecture

- **main.rs**: Server entry point and connection listener
- **socks5.rs**: SOCKS5 protocol implementation and data forwarding
- **Asynchronous Design**: Uses Tokio for non-blocking I/O operations
- **Concurrent Connections**: Handles multiple client connections simultaneously

## Limitations

- Only supports CONNECT command (no BIND or UDP ASSOCIATE)
- No authentication methods other than "no authentication"
- No connection limits or rate limiting
- Basic error handling

## Development

### Building for Development

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This project is open source. Please check the license file for details.

## Disclaimer

This SOCKS5 proxy server is a basic implementation for educational and development purposes. It may not be suitable for production use without additional security measures, authentication, and monitoring features.
