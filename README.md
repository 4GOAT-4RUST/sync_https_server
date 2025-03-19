# SYNC HTTP DELAY SERVER 

## Description
An Rust server From scratch that accepts HTTP requests , extracts query parameters decodes the Base64 -encoded message and responds with the decoded message wiht a delay (in miliseconds).


## Features

-  Safe thread queue  
-  Multi-threaded system 
## Prerequisites

Before running this project, ensure you have:
- [Rust & Cargo](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/) <br>
Installed.
## Installation & Running
- **Clone The Repository**
```sh  
git clone https://github.com/4GOAT-4RUST/sync_https_server.git  
```
- **Navigate to The Project directory**
```sh
cd sync_https_server
```
- **Run Directly with Cargo**
```sh
cargo run
```
- **Send a Request**
The server expects a **GET** request with two query parameters:

```
GET /?msg=<Base64-encoded string>&delay=<milliseconds>
```

| Parameter | Description                                     | Example                              |
|-----------|-------------------------------------------------|--------------------------------------|
| `msg`     | A Base64-encoded string to decode              | `SGVsbG8gd29ybGQ=` (for "Hello world") |
| `delay`   | The delay (in milliseconds) before responding  | `2000` (for a 2-second delay)       |

**Run Using Docker**
-  **Build the Docker Image**
```sh
    docker build -t your-image-name .
```
- **Run the Container**
```sh
    docker run --rm -p 8080:8080 your-image-name
```
- **Send Request(Docker)**
```sh
curl "http://localhost:8080/?msg=SGVsbG8gd29ybGQ=&delay=2000"
```
**Optionally you can still Use docker Compose**

Build the Docker image , Start the server on port 8080 using
```sh
docker-compose up --build
```
## Preview


## License
This project is licensed under the MIT License. See the [LICENSE](https://opensource.org/licenses/MIT) file for more details.