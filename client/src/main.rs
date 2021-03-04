#[macro_use]
extern crate serde;
extern crate toml;
extern crate rand;
extern crate hex;

use hex::FromHex;
use std::io::{self, Read};
use std::fs::File;
use mozaic_core::Token;
use mozaic_core::client::simple_client::{ClientParams, simple_client};
use clap::{App, Arg};

#[derive(Serialize, Deserialize)]
struct Config {
    server: String,
    frontend: String,
    bot_token: Option<String>,
    bot_argv: Vec<String>,
}

async fn run(config_path: &str) {
    let config = match read_config(&config_path) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("failed to read {}:", config_path);
            eprintln!("{}", err);
            return;
        }
    };

    // TODO: add this when it actually works
    // println!("Click to go to the lobby: {}/lobbies/hub?token={}", frontend, str_token);

    let token = match get_token(&config) {
        Ok(token) => token,
        Err(err) => {
            eprintln!("error parsing token:");
            eprintln!("{}", err);
            return;
        }
    };

    let params = ClientParams {
        server: config.server,
        argv: config.bot_argv,
        token
    };
    simple_client(params).await;
}

#[tokio::main]
async fn main() {
    let matches = App::new("planetwars-client")
        .version("0.1")
        .author("Zeus WPI")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .subcommand(App::new("run-client")
            .about("Runs the planetwars client")
            .long_about("Runs the planetwars client [aliases: run, client]")
            .alias("run")
            .alias("client")
            .arg(Arg::new("config-file")
                .about("config file to use")
                .required(true)))
        .subcommand(App::new("generate-token"))
        .get_matches();
    
    match matches.subcommand() {
        Some(("generate-token", _)) => {
            let token: Token = rand::random();
            let str_token = hex::encode(&token);
            println!("{}", str_token);    
        }
        Some(("run-client", matches)) => {
            let config_path = matches.value_of("config-file").unwrap();
            run(config_path).await;        
        }
        None => { eprint!("no command specified") }
        _ => unreachable!()
    }
}

fn read_config(config_path: &str) -> Result<Config, io::Error> {
    let mut config_file = File::open(config_path)?;
    let mut buf = String::new();
    config_file.read_to_string(&mut buf)?;
    let config = toml::from_str(&buf)?;
    return Ok(config);
}

fn get_token(config: &Config) -> Result<Token, hex::FromHexError> {
    if let Some(ref token_str) = config.bot_token {
        Token::from_hex(token_str)
    } else {
        // generate a temporary token
        let token: Token = rand::random();
        let token_str = hex::encode(&token);
        println!("No token found in configuration file.");
        println!("Using temporary token {}\n", token_str);
        println!("To keep using this token, add the line below to your configuration file");
        println!("bot_token = \"{}\"", token_str);
        Ok(token)
    }
}