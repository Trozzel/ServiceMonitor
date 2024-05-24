# Service Monitor
A simple way to monitor services

## Setup
1. Establish SSH key based conncection between the server (i.e. the host on which you installed this software) and the host machines that you want to monitor.

On the server, if you haven't yet generated a public key on the server:
```bash
ssh-keygen -t rsa
```
You can continue to hit `<Enter>` until you see the randomart imgae appear.

Now, append the contents of that file to the host's (which is also the SSH server in this case) `authorized_keys`
```bash
ssh <user>@<host> cat svcmon_id_rsa.pub >> ~/.ssh/authorized_keys
```

2. Edit the `config/hosts.json` file with the hosts and services that you want to monitor
Edit the `config.hosts.json` file with the hostnames and the services that you want to monitor.

## Run the server
From the project base directory, enter:
```bash
./runserver.sh
```
This will build the React / Javascript frontend code first, compile the Rust server, and then run the server.

That's it!
