# Planetwars starterpack

The entrypoint to create your own bot for our AI planetwars game.

## Prebuilt client
A prebuilt binary for linux is available on the github releases page for this repository.
If this doesn't work for you, follow the instructions below to build it yourself.

To start the client, run
```bash
./planetwars-client run config.toml
```
an example `config.toml` file can be found in this repository.

## Building the client

Get immediatly started with just a few steps.

### Required tools

- curl
- git
- gcc (or any other C compiler of your choice)
- rust

### Installing rust

If you don't have rust installed already, run this command to install it:
```bash
curl --proto '=https' --tlmsv1.2 -sSf https://sh.rustup.rs | sh
```
It should ask what version to install. The default is fine, so just press enter.

Now, activate rust in your current shell:

```bash
source $HOME/.cargo/env
```

### Building the client

Next, build the client

```bash
cd planetwars-starterpack/client
cargo build
```

Now you can run 
```bash
cargo run client ../config.toml`
```

(sidenote: for the prebuilt client we used the `run` subcommand, but here we use the `client` subcommand.
These are aliasses, the author got upset typing `cargo run run` in their terminal.)

## Joining a lobby
Using the method of your chosing described above, you should now have your client running.

You should see some output stating a bot token was generated for you. We will use this
token in the next step to identify your user.
You probably want to add this token to your configuration file to make it permanent,
as per the instructions in your terminal. If you don't do this, a new token will
be generated the next time you run the client, so you'll have to authenticate again.

If you already have a lobby URL you wish to join, simply navigate there. If not,
you can create a new lobby at [games.zeus.gent](https://games.zeus.gent). 

1. Click the big blue 'join lobby' button
2. Enter your desired username, and the token you got in the last step.

## Playing matches

You can now challenge other players in the lobby by using the 'new game' button.
Once all players have accepted your proposed match, you can start it, and the match
should appear in the match list shortly.
Note that one player can be in a match multiple times, so you can play against your own bot if you like.

## How does this work?

The client you are running on your system connects to the game server.
When a match starts, the game server will simply request your client to start a bot for the given player slot.
Your client will then execute the bot command specified in your configuration file to play the match.
Messages from the game server will be relayed to stdin of that bot process, and lines written to stdout will be
sent back to the gameserver.

## Writing your own bot

We provided the `simple.py` script as a very simple bot to show you how it works. Feel free to use this file as a
starting point, as inspiration, or disregard it altogether and come up with your own bot.
Anything you can run on your own system will work as a bot, it just has to read from stdin and write to stdout.
You can edit the `config.toml` file to set the command that will be used to start your bot.

Be aware that it is possible that your bot will be running multiple times simultaneously,
keep this in mind when e.g. logging output to a file.

You can find a good overview of the game rules on the [old PlanetWars website](https://planetwars.zeus.gent/info/).

Good luck and have fun!

## FAQ

> How do I play against other people?

Simply send them the url of your lobby. They will then have to enter their own username en token to enter.

> How do I run my own bot and not `simple.py`?

Just edit the `config.toml` file and change the command after `bot-command = ...` into your own command.

> I want to join different lobbies with my bot.

You can use the same token used in the first lobby for other lobbies.