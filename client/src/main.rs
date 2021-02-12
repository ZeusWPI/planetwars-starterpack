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

const CONFIG_FILE: &'static str = "config.toml";
const TOKEN_FILE: &'static str = "token";

#[derive(Serialize, Deserialize)]
struct Config {
    server: String,
    bot_argv: Vec<String>,
}

#[tokio::main]
async fn main() {
    let config = match read_config() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("error reading {}:", CONFIG_FILE);
            eprintln!("{}", err);
            return;
        }
    };

    let token = match get_token() {
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

fn read_config() -> Result<Config, io::Error> {
    let mut config_file = File::open(CONFIG_FILE)?;
    let mut buf = String::new();
    config_file.read_to_string(&mut buf)?;
    let config = toml::from_str(&buf)?;
    return Ok(config);
}

fn get_token() -> Result<Token, io::Error> {
    let token_file_path = Path::new(TOKEN_FILE);
    if token_file_path.exists() {
        let mut token_file = File::open(token_file_path)?;
        let mut buf = String::new();
        token_file.read_to_string(&mut buf)?;
        // todo: return an error here instead of panicking
        let token = Token::from_hex(buf)
            .expect("failed to parse token");
        println!("using token {} from token file", hex::encode(&token));
        return Ok(token);
    } else {
        let token: Token = rand::random();
        println!("using generated token {}", hex::encode(&token));
        let mut token_file = File::create(token_file_path)?;
        write!(token_file, "{}", hex::encode(&token))?;
        println!("saved token to token file");
        return Ok(token)
    }
}