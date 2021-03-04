#[macro_use]
extern crate serde;
extern crate toml;
extern crate rand;
extern crate hex;

use hex::FromHex;
use io::Write;
use std::{io, io::Read};
use std::fs::File;
use std::path::Path;
use mozaic_core::Token;
use mozaic_core::client::simple_client::{ClientParams, simple_client};
use clap::{App, Arg};

const DEFAULT_CONFIG_FILE: &'static str = "config.toml";


#[derive(Serialize, Deserialize)]
struct Config {
    server: String,
    frontend: String,
    bot_argv: Vec<String>,
}

async fn run(config_path: &str) {
    let config = match read_config(&config_path) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("error reading {}:", config_path);
            eprintln!("{}", err);
            return;
        }
    };


    let token = match get_token(config.frontend) {
        Ok(token) => token,
        Err(err) => {
            eprintln!("error fetching token:");
            eprintln!("{}", err);
            return;
        }
    };

    let params = ClientParams {
        server: config.server,
        token: token,
        argv: config.bot_argv,
    };
    println!("connecting to server");
    simple_client(params).await;
}

#[tokio::main]
async fn main() {
    let matches = App::new("planetwars-client")
        .version("0.1")
        .author("Zeus WPI")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .default_value(DEFAULT_CONFIG_FILE)
            .takes_value(true))
        .subcommand(App::new("generate-token"))
        .get_matches();
    
    match matches.subcommand() {
        Some(("generate-token", _)) => {
            let token: Token = rand::random();
            let str_token = hex::encode(&token);
            println!("{}", str_token);    
        }
        _ => {
            let config_path = matches.value_of("config").unwrap();
            run(config_path).await;        
        }
    }
}

fn read_config(config_path: &str) -> Result<Config, io::Error> {
    let mut config_file = File::open(config_path)?;
    let mut buf = String::new();
    config_file.read_to_string(&mut buf)?;
    let config = toml::from_str(&buf)?;
    return Ok(config);
}

fn get_token(frontend: String) -> Result<Token, io::Error> {
    let token_file_path = Path::new("tokenfile");
    if token_file_path.exists() {
        let mut token_file = File::open(token_file_path)?;
        let mut buf = String::new();
        token_file.read_to_string(&mut buf)?;
        // todo: return an error here instead of panicking
        let token = Token::from_hex(buf)
            .expect("failed to parse token");
        let str_token = hex::encode(&token);
        println!("using token {} from token file", str_token);
        println!("Click to go to the lobby: {}/lobbies/hub?token={}", frontend, str_token);
        return Ok(token);
    } else {
        let token: Token = rand::random();
        let str_token = hex::encode(&token);
        println!("using generated token {}", str_token);
        println!("Click to go to the lobby: {}/lobbies/hub?token={}", frontend, str_token);
        let mut token_file = File::create(token_file_path)?;
        write!(token_file, "{}", hex::encode(&token))?;
        println!("saved token to token file");
        return Ok(token)
    }
}