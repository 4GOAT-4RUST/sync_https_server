# 🚀 **Building a Web Server**  

## **📌 Overview**  
This web server is designed to **handle incoming requests** and send responses based on the provided **payload** and **delay**.  

### **Key Features**  
✅ **Multi-threaded request handling** – Uses threads to process requests concurrently.  
✅ **Configurable delay** – The server introduces a delay before responding.  
✅ **Thread-safe queue** – Manages incoming requests efficiently.  

---  

# 🛠️ **Project Setup**  

### **📥 Cloning Our Repository**  
To have this project locally, run the following commands:  
```sh  
git clone https://github.com/4GOAT-4RUST/sync_https_server.git  
cd sync_https_server  
```

---

# ✅✅✅  Testing Web Server  

Unit tests (testing of each function) and integration tests (testing different functionalities of the code) were carried out automatically in the CI/CD pipeline. Here are some screenshots of the tests:  

### **1. Unit tests**  
![Test](/images/image1.png)  
![Test](/images/image2.png)  

### **2. Integration tests**  
---  

# ✨ How It Works And How You Can Use ✨  

## **1. How It Works**  

This project builds a program that extracts an encoded message found in the **payload** of a request and the **delay** value. It then converts the payload using **256-character encoding**.  

It introduces a delay based on the request and then sends the result after the delay.  

The server can receive multiple requests and handle execution concurrently using a **thread pool of max size 8 threads**. Other requests are **queued** and processed when a worker becomes available.  

## **2. How to Use It**  

### 📌 **Running the Server Using Docker**  

#### **A. By Building the Docker Image Locally**  
After cloning the repo, you can build the Dockerfile and run the image using:  

***_Building the Image_***  
```sh  
cd sync_https_server  
docker build -t sync_https_server .  
```

***_Running the Image_***  
```sh  
docker run --rm -p 8080:8080 sync_https_server  
```

On another terminal, run:  
```sh  
curl -X POST -d "payload=SGVsbG8ui29ywGQ=" -d "delay=5" http://127.0.0.1:8080/decode  
```

#### **B. Pulling and Running the Image from GitHub Container Registry (GHCR)**  
Instead of building locally, you can pull the pre-built image from GHCR:  

***_Pulling the Image_***  
```sh  
docker pull ghcr.io/4goat-4rust/sync_https_server:latest  
```

***_Running the Image_***  
```sh  
docker run --rm -p 8080:8080 ghcr.io/4goat-4rust/sync_https_server:latest  
```

---  

### ✅ **Examples of Running Docker Container**  

![Test](images/Screenshot%20from%202025-03-12%2009-19-06.png)  
![Test](images/Screenshot%20from%202025-03-12%2011-58-21.png)  

---  

### ❌ **Examples of Failed Project Runs**  

![Failed](images/Screenshot%20from%202025-03-12%2010-00-57.png)  

Another one:  

![Failed](images/Screenshot%20from%202025-03-12%2010-01-07.png)  

---  

Now, whether you **build the image locally** or **pull it from GHCR**, you can easily run the web server with Docker! 🚀  

