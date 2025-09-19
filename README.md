# Simple API Demo for Otoroshi on Clever Cloud

This project is a simple Rust-based API designed as a learning tool for integrating services with the [Otoroshi](https://www.otoroshi.io/) reverse proxy, specifically for deployments on [Clever Cloud](https://www.clever-cloud.com/).

The application provides a few basic endpoints and is configured to run seamlessly on the Clever Cloud platform. The goal is to provide a straightforward backend service that you can place behind an Otoroshi instance to experiment with routing, security, and other API management features.

## Overview

*   **Language:** Rust
*   **Framework:** actix-web
*   **Platform:** Clever Cloud

This project intentionally omits configurations for Docker, Kubernetes, or other containerization platforms to maintain a clear focus on a direct Clever Cloud deployment.

## Network Configuration

The application server is configured to listen on the following TCP port:

*   **Port:** `8080`

The server binds to `0.0.0.0:8080`. When deploying on Clever Cloud, the platform will automatically map incoming traffic from its load balancer to this port. You do not need to expose it manually.

In your Otoroshi service configuration, you will need to create a target that points to your Clever Cloud application's hostname on port `8080`.

## API Endpoints

The following endpoints are available for testing:

*   `GET /`: Returns a simple welcome message.
*   `GET /hello`: Returns a "Hello, World!" style message.
*   `POST /echo`: A simple echo service that returns the JSON body it receives.

**Example usage with `curl`:**

```bash
# Get the welcome message
curl https://<your-clever-cloud-app-domain>/

# Get the hello message
curl https://<your-clever-cloud-app-domain>/hello

# Echo a JSON payload
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"message": "testing echo"}' \
  https://<your-clever-cloud-app-domain>/echo
```

## Deployment on Clever Cloud

1.  Create a new "Rust" application on your Clever Cloud account.
2.  Link your Git repository to the application.
3.  Push your code to the main branch.

Clever Cloud will automatically detect the `Cargo.toml` file, build the Rust project, and run the resulting binary. No further configuration is required.