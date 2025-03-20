# SYNC HTTP DELAY SERVER 

## Description
A Rust server from scratch that accepts HTTP requests, extracts query parameters, decodes the Base64-encoded message, and responds with the decoded message with a delay (in milliseconds).

## Features

- Safe thread queue  
- Multi-threaded system 

## Prerequisites

Before running this project, ensure you have:
- [Rust & Cargo](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/) <br>
Installed.

## Installation & Running

### Using Cargo
```sh  
git clone https://github.com/4GOAT-4RUST/sync_https_server.git  # Clone repo
cd sync_https_server  # Enter project directory
cargo run  # Build and run the server
```

### Using Docker
#### Build and Run
```sh
docker build -t sync_https_server .  # Build Docker image
docker run --rm -p 8080:8080 sync_https_server  # Run container
```
#### (Optional) Pull Prebuilt Image
```sh
docker pull ghcr.io/4goat-4rust/sync_https_server:slim  # Use prebuilt image
```
#### Using Docker Compose
```sh
docker-compose up --build  # Build and start with Compose
```
## Sending Requests

The server expects a **GET** request with two query parameters:

```
GET /?msg=<Base64-encoded string>&delay=<milliseconds>
```

| Parameter | Description                                     | Example                              |
|-----------|-------------------------------------------------|--------------------------------------|
| `msg`     | A Base64-encoded string to decode              | `SGVsbG8gd29ybGQ=` (for "Hello world") |
| `delay`   | The delay (in milliseconds) before responding  | `2000` (for a 2-second delay)       |

**Example Request:**
```sh
curl "http://localhost:8080/?msg=SGVsbG8gd29ybGQ=&delay=2000"
```


## Preview

![Preview](/images/Screenshot%20from%202025-03-19%2010-31-10.png)

## License
This project is licensed under the MIT License. See the [LICENSE](https://github.com/4GOAT-4RUST/sync_https_server/blob/dev/LICENSE) file for more details.

