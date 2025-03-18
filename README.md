#  **Multithreaded Sync Http Server**  


This web server is designed to **handle incoming requests** and send responses based on the provided **payload** and **delay**.  

The server can receive multiple requests and handle execution concurrently using a **thread pool of max size 8 threads**. Other requests are **queued** and processed when a worker becomes available. 

### **Key Features**  
- **Multi-threaded request handling** – Uses threads to process requests concurrently.  
- **Configurable delay** – The server introduces a delay before responding.  
- **Thread-safe queue** – Manages incoming requests efficiently.  

---  

Build and Run
-------------
####  Cloning The Repository
To have this project locally, run the following commands:  
```sh  
git clone https://github.com/4GOAT-4RUST/sync_https_server.git  
cd sync_https_server  
```
#### Build and Run
To build this project run the following commands
```sh
cargo build
cargo run --release
```

#### Run Using Docker

To run this project using [docker](https://docs.docker.com/get-started/docker-overview/) you need to have docker running on your local machine.
You can either use the image on [ghcr](https://github.com/4GOAT-4RUST/sync_https_server/pkgs/container/sync_https_server) or build it locally 

**Using docker image**

- **Building and running docker image locally**

```sh
    docker build -t your-image-name .
    docker run --rm -p 8080:8080 your-image-name
```

#### running without Cloning     
```sh
docker pull ghcr.io/4goat-4rust/sync_https_server:slim 
docker run --rm -p 8080:8080 ghcr.io/4goats-4rust/sync_https_server:slim 
```

#### structure of A request 
```sh
curl -X <METHOD> <URL> -H "<HEADER>" -d "<DATA>"
```

example
```sh  
curl -X POST -d "payload=SGVsbG8ui29ywGQ=" -d "delay=5" http://127.0.0.1:8080/decode  
```

> *NB*: Note that if you aliased docker in you in you host machine to docker in your vm use your vm IP address when sending the request to the server

#### Visualizer 

![Image](/images/running_http_server.png)
![Image](/images/http_request.png)

[For more details on this project visit our docs]()

## License
This project is licensed under the MIT .
You are free to use, modify, and distribute it under these terms.

Contact 
[Jagoum](https://github.com/Jagoum)
[Usher](https://github.com/USHER-PB)
[Emmanuel](https://github.com/Donemmanuelo)
[Onel](https://github.com/onelrian)
for issues , Open issue