use discord::Discord;
use clap::Parser;
use discord::model::ServerId;
use dotenv::dotenv;
use std::str::FromStr;
use std::u64;

extern crate discord;

// TODO: Better validation of Discord token.
const BOT_TOKEN_LENGTH: usize = 72;

/// Tri-Cities Vertex Discord Bot
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    /// Your bot token you got from Discord Developer portal.
    #[arg(short, long, env("BOT_TOKEN"))]
    pub bot_token: String,

    /// Your Discord Server ID that you want to interact with
    /// TODO: This will probably change if/when this bot becomes more generic.
    #[arg(short, long, env("SERVER_ID"))]
    pub server_id: String,
    //
    // /// Your Discord App's Public Key
    // /// TODO: Is this needed?
    // #[arg(short = "k", long, env("DISCORD_PUBLIC_KEY"))]
    // pub discord_public_key: String,
}

impl Config {
    pub fn from_env_and_args() -> Self {
        dotenv().ok();
        Self::parse()
    }

    /// Creates a new `Config` instance with default values.
    pub fn new(bot_token: String, server_id: String) -> Self {
        Self {
            bot_token,
            server_id,
        }
    }
}

fn get_discord(bot_token: &str) -> Discord {
    if bot_token.len() < BOT_TOKEN_LENGTH {
        panic!("invalid bot token");
    }


    return match Discord::from_bot_token(bot_token) {
        Ok(discord) => discord,
        Err(err) => panic!("error happened: {}", err)
    };
}

/// Extracts the server id from the config
fn get_server_id(config: Config) -> u64 {
    let num = u64::from_str(config.server_id.as_str()).unwrap();
    return num;
}

fn main() {
    let cfg = Config::from_env_and_args();
    let discord = get_discord(cfg.bot_token.as_str());
    let server_id = ServerId(self::get_server_id(cfg));
    let server = discord.get_server(server_id).unwrap();
    println!("{}", server.name);
}

#[cfg(test)]
mod tests {
    use crate::{Config, get_discord, get_server_id};

    #[test]
    #[should_panic]
    fn invalid_bot_token_errors() {
        get_discord("invalid");
    }

    #[test]
    fn valid_bot_token_no_error() {
        // Our only validation rule right now is length
        get_discord("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    }

    #[test]
    fn test_get_server_id() {
        let config = Config::new("123".to_string(), "456".to_string());
        let expected_result = 456;
        assert_eq!(expected_result, get_server_id(config));
    }
}
