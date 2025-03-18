#  **Multithreaded Sync Http Server**  


This web server is designed to **handle incoming requests** and send responses based on the provided **payload** and **delay**.  
This project builds a program that extracts an encoded message found in the **payload** of a request and the **delay** value. It then converts the payload using **256-character encoding**.  
It introduces a delay based on the request and then sends the result after the delay.  
The server can receive multiple requests and handle execution concurrently using a **thread pool of max size 8 threads**. Other requests are **queued** and processed when a worker becomes available. 

### **Key Features**  
- **Multi-threaded request handling** – Uses threads to process requests concurrently.  
- **Configurable delay** – The server introduces a delay before responding.  
- **Thread-safe queue** – Manages incoming requests efficiently.  

---  

Build and Run
-------------
####  Cloning Our Repository
To have this project locally, run the following commands:  
```sh  
git clone https://github.com/4GOAT-4RUST/sync_https_server.git  
cd sync_https_server  
```
#### Build and Run Repo
To build this project run the following commands
```sh
cargo test --verbose # to test the project
cargo check # to check if it build correctly
cargo build
cargo run --release

```

#### Run Using Docker

To run this project using [docker](https://docs.docker.com/get-started/docker-overview/) you need to have [docker](https://docs.docker.com/get-started/docker-overview/) running on your local machine.
You can either use the image on [ghcr](https://github.com/4GOAT-4RUST/sync_https_server/pkgs/container/sync_https_server) or build it locally 
- **Using docker image**


```sh
docker pull ghcr.io/4goat-4rust/sync_https_server:slim 
docker run --rm -p 8080:8080 ghcr.io/4goats-4rust/sync_https_server:slim 
```

To send a request to the server from another terminal 
```sh
    curl -X POST -d "payload=Q29uY3VycmVuY3kgaXMgcG93ZXJmdWw=" -d "delay=12" http://127.0.0.1:8080/decode
```
- **Building and running docker image locally**

```sh
    docker build -t your-image-name .
    docker run --rm -p 8080:8080 your-image-name
```

On another terminal, run this command to send a request to the server with a desired payload and delay:  
```sh  
curl -X POST -d "payload=SGVsbG8ui29ywGQ=" -d "delay=5" http://127.0.0.1:8080/decode  
```
> *NB*: Note that if you aliased docker in you in you host machine to docker in your vm use your vm IP address when sending the request to the server

#### Integration tests  
![Test](/images/image2.png) 

[For more details on this project visit our docs]()
