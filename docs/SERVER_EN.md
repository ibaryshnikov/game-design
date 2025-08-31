# Game server

Server-side part of the game, which can also serve web client


## Web client

To test game server it's required to build the web client:

```bash
cd client-web
./build.sh
```


## Running the server

For a local test it's enough to do:

```bash
cd server
cargo run
```

and navigate to `http://localhost:8080/game`


## Uploading and running on a remote server

For this it's required to build the web client and the game server
in optimized mode (release mode):

```bash
# build the client
cd client-web
./release_build.sh
# build the server
cd ../server
cargo build --release
```

After this we have all we need to upload the game on a remote server.
We can start uploading game files:

```bash
# First we need to create a few folders on a remote server.
# Connect with ssh and execute
mkdir /game
cd game
mkdir bin
mkdir client-web
# Afterwards for convenience open a separate terminal.
# Upload game data files (abilities, enemies, levels, ...)
scp -r data user@ip:/game
# Upload game server binary and configuration file
cd server
scp target/release/server user@ip:/game
scp config.toml user@ip:/game
# Upload web client
cd ../client-web
scp -r dist user@ip:/game
# Moving files on a remote server into the required folders.
# For this return to the terminal where we did ssh to the server, and execute:
mv server bin
mv config.toml bin
mv dist client-web
# Change port number in config.toml file to 80
# Test configuration file
cat bin/config.toml 
# Should print: port = 80
```

After uploading all the files to a remote server we are ready to run the game server. 

```bash
# Run the game server
cd bin
./server
# To test navigate to http://server_ip_address/game
# Use nohup so that the game server don't stop after you close ssh session
nohup ./server > /dev/null 2>&1 &
# Check the game server process id
ps ax | grep server
