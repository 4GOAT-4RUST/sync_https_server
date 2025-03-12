Here’s a more **polished and professional** version of your document:  

---

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
To have this project locally , run the following commands:  
```sh
git clone https://github.com/4GOAT-4RUST/sync_https_server.git
cd sync_https_server
```

# ✅✅✅  Testing  Web Server 

The unit test that is testing of each function and intergration tests that is test for different functionality of the code were carried out automatically in ci/cd pipeline and here are some screenshot of the tests carried:
 
**1.Unit tests**
![Test](/src/image1.png)
![Test](/src/image2.png)

**2.Intergration tests**
---

# ✨ How It Works And How You Can Use ✨

 ## 1. How it works 

    This projects builds a program that extracts an encode message which is found in the  payload  of a request and the delay also and then convert the payload to using 256 character encoding . 
    
    It then delays for the amount of delay in the request and then send the Result after that time delayed.

    The server can then receive any number of request and handle the execution of the processes simultaneously and in a threadpool of max size 8 thread . The other request are kept in a queue where they wait for a worker to become free.  


 ## 2. How to use it 
### 📌 By Using Our Dockerfile

After cloning the repo you can just simply build the docker file and run the image using
***_Buildingg Image_*** 
```sh
cd sync_https_server
docker build -t sync_https_server .
```
***_Running Image_***
```sh
docker run --rm sync_https_server
```
On another terminal run 
```sh
 curl -X POST -d "payload=SGVsbG8ui29ywGQ=" -d "delay=5" http://127.0.0.1:8080/decode
```

📌 Using Our GHCR

You can also get the image of this project from ghcr.

✅ Examples of running docker container

![Test](./Screenshot%20from%202025-03-12%2009-19-06.png)

❌ Examples of failed project runs

![Failed](./Screenshot%20from%202025-03-12%2010-00-57.png)

Another one

![Failed](./Screenshot%20from%202025-03-12%2010-01-07.png)


