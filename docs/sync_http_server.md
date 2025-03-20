# OVERVIEW 
This is a synchronous http server in rust, and this server provide an appropiate way of handling http request and reponses  using synchronous code, rather than asynchronous code. This means that this server processes each request in a blocking manner, using threads to handle multiple request concurrently.

## Why Sync Http Server
As most developers turns to avoid complixity of asynchronous http server, this synchronous http server is usefull because it include little or no abstraction, with zero dependencies, only standard library is used in this server.
### Key features:
- Safe thread queue: So the queue is where all incomming request from the client are stored, before been proccessed accordingly. 
- Multi-threaded system: This server make use of threadpool which is set a of previousely spawned threads that enable us to run tasks in parallel without having to spawn new threads for a task. Also this server make use of multiple thread workers to listen to a maximum of eight http request at a time where in each worker thread has and id which allow easy identification.
### How it works
  ![clientserver](/images/image1.png)

This diagram above illustrate a http request been sent to the server, what happen is that, when this request arrive the server, the request is immediately stored in the queue, a thread workers is then assigned to a request, and varrious threads are assigned to different tasks to be executed in that request. Each request made to our server has two parameters that is the payload and the delay, the payload is simply a base-64-encoded message that is 
sent to the server, so that it decode this message and sent it back as reponse in human readable form, delay is simply the time elapsed for which the server has to wait before sending back each response. This server works the same way for requests for multiple clients multiple ciients.
