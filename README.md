# Simple API Demo

A simple Rust-based API demonstration project designed to learn Rust deployment on CleverCloud PaaS and test Otoroshi reverse proxy features.

## Project Overview

This project implements two HTTP servers:
- Main server (PORT, default: 8080): Simple hello world endpoint
- Application server (PORT_APP, default: 4242): Multiple endpoints with JSON responses

### Endpoints

Main Server:
- `GET /`: Returns "Hello world!"

Application Server:
- `GET /`: Returns `{"status": "ok"}`
- `GET /public`: Returns `{"message": "public route"}`
- `GET /private`: Returns `{"message": "private and protected route"}`

## Environment Variables

- `PORT`: Main server port (default: 8080)
- `PORT_APP`: Application server port (default: 4242)
- `RUST_LOG`: Log level (recommended: "info")

## Running the Project

1. Clone the repository
2. Build the project:
```bash
cargo build
```

3. Run the servers:
```bash
Cargo RUST_LOG=info run
```

## Deployment on CleverCloud

This project is designed to be deployed on CleverCloud PaaS platform. It serves as a learning example for:
- Rust application deployment
- Multi-port service configuration
- Environment variable usage
- Logging configuration

## Otoroshi Testing

The dual-server setup allows testing various Otoroshi reverse proxy features:
- Route mapping
- Service discovery
- Load balancing
- Access control
- API gateway features

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Contributing

Feel free to submit issues and enhancement requests!
