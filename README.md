# Planetwars starterpack

The entrypoint to create your own bot for our AI planetwars game.

## Quick setup

Get immediatly started with just a few steps.

Open a terminal where you want to place your code. Now run
```
git clone https://github.com/ZeusWPI/planetwars-starterpack.git
cd planetwars-starterpack
cargo run
```

Now browse to [pw.zeus.gent](https://pw.zeus.gent). 

1. Click `Create a lobby`
2. Enter a username of your choice
3. Enter the token that was printed in your terminal
## What now?

You're now connected to our planetwars server and can play games against other people in your lobby.

TODO explanation how the frontend works and how you play matches

## What am I running?

You are running a client that does all the complex networking for you.
It will start up your bot and run the matches you start in your browser.

You can change the command that runs the bot to your own command by editing the `config.toml` file.
## FAQ

> How do I play against other people?

Simply send them the url of your lobby. They will then have to enter their own username en token to enter.

> How do I run my own bot and not `simple.py`?

Just edit the `config.toml` file and change the command after `bot-command = ...` into your own command.
