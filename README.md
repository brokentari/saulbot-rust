# saulbot-rust

The saulbot-rust is a reimplementation of the saulbot, which was originally written in Javascript using the discord.js library. A majority of the functionality is implemented here along some extra features such as storing message in a database rather than a JSON file. 


## Local Installation
Clone the repo
```bash
git clone https://github.com/brokentari/saulbot-rust.git
```

Run the program
```bash
cargo run
```

## Deployment
The repo also contains a Dockerfile that allows you to create an Docker image if you wish to deploy it elsewhere, such as a Kubernetes cluster.
