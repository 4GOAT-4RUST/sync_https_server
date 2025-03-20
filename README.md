# SYNC HTTP DELAY SERVER 

This Server accepts HTTP GET requests , delays the responds before sending it back.

## Features

- Threadpool 
- Multi-threaded system 


## Build & Run

### Prerequisites

Before running this project, ensure you have:
- [Rust & Cargo](https://www.rust-lang.org/tools/install)
Installed

---
```sh  
git clone https://github.com/4GOAT-4RUST/sync_https_server.git  # Clone repo
cd sync_https_server  # Enter project directory
cargo run  # Build and run the server
```

#### Using Docker
- Optionally you can use [Docker](https://docs.docker.com/get-docker/) to build and run the server 
 
```sh
docker build -t sync_https_server .  # Build Docker image
docker run --rm -p 8080:8080 sync_https_server  # Run container
```
OR

 - [Docker compose](https://docs.docker.com/compose/) <br>

```sh 
docker-compose up --build  # Build and start with Compose

```
- also you could simply Pull the Prebuilt Image
```sh
docker pull ghcr.io/4goat-4rust/sync_https_server:slim  # Use prebuilt image
```
## Sending Requests

The server expects a **GET** request with two query parameters:

```
GET http://localhost:8080/?msg=<Base64-encoded string>&delay=<milliseconds>
```

| Parameter | Description                                     | Example                              |
|-----------|-------------------------------------------------|--------------------------------------|
| `msg`     | A Base64-encoded string to decode              | `SGVsbG8gd29ybGQ=` (for "Hello world") |
| `delay`   | The delay (in milliseconds) before responding  | `2000` (for a 2-second delay)       |

**Example Request:**
```sh
curl "http://127.0.0.1:8080/?payload=Q29uY3VycmVuY3kgaXMgcG93ZXJmdWw=&delay=10"
```

## Preview

![Preview](/images/Screenshot%20from%202025-03-20%2016-35-10.png)

## License
This project is licensed under the MIT License. See the [LICENSE](https://github.com/4GOAT-4RUST/sync_https_server/blob/dev/LICENSE) file for more details.

