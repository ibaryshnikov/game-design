# Game creation project

Contents:
- [overview](#overview)
- [constructor](#constructor)
- [server](#server)
- [client](#client):
  - [web](#web-client)
  - [desktop](#desktop-client)
  - [mobile](#mobile-client)


## Overview

Current project is a set of tools for game development of
a particular genre. Main direction is 2D/3D action, bullet hell, MMO.
To make your game:
- create a set of abilities
- create one or several enemies (Npc), and give them abilities
- create one or several levels

Afterwards you would be able to play the game in single mode.
For multiplayer mode:
- set up a server (usually can get one from a VPS or hosting provider)
- upload game data and game server
- run game server on a remote server
- connect to the game server from one of the [clients](#client)


## Constructor

Application for making and editing abilities, enemies and game levels.
To start the constructor:
```bash
cd editor
cargo run
```


## Server

Game server for multiplayer mode. To run the server:
```bash
cd server
cargo run
```


## Client

After you created abilities, enemies and levels in the editor you can
run the game in either single player or multiplayer modes.
Below is a list of clients.


### Web client

Client to play the game a web browser. At the moment of writing it supports only
multiplayer mode. To enter the game from the browser you need to run the server
and navigate to a corresponding web page. For example, if you are testing it locally,
the address would be `http://localhost:8080/game`. And if you are running
a game server on a remote server, the address is `http://server_ip/game`.


### Desktop client

Client for desktop OS such as Windows, macOS, Linux. It can run the game
in either single player or multiplayer modes. More detailed about setting
up and running the desktop client will be written in future versions of
this project, together with an addition of new features.


### Mobile client

Currently in development
