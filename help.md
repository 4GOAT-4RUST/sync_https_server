### Guide on how to run the image in multipass virtual machine

**How to install multipass virtual machine**

see [help](https://documentation.ubuntu.com/server/how-to/virtualisation/multipass/index.html)

**How to run the image in a container in a virtual machine**

Before logging in the virtual machine, you can run this command to know your ip address: 
```sh
multipass list 
```
OR

```sh
multipass info
```

After login in the virtual machine and pulling just run the command:
```sh
docker run --rm -p 8080 ghcr.io/4goat-4rust/sync_https_server:slim
```

Open another terminal in your local machine then you can run this command to send the request:
```sh
curl "http://<my_vm_ip>:8080/?msg=SGVsbG8gd29ybGQ=&delay=2000"
```

Remember to replace <my_vm_ip> with ip address of your multipass instance that you got from the command above.