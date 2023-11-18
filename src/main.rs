use discord::Discord;
use clap::Parser;
use dotenv::dotenv;

extern crate discord;

// TODO: Better validation of Discord token.
const BOT_TOKEN_LENGTH: usize = 72;

/// Tri-Cities Vertex Discord Bot
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    /// Your bot token you got from Discord Developer portal.
    #[arg(short, long, env("BOT_TOKEN"))]
    pub bot_token: String
}

impl Config {
    pub fn from_env_and_args() -> Self {
        dotenv().ok();
        Self::parse()
    }
}

fn get_discord(bot_token: &str) -> Discord {
    if bot_token.len() < BOT_TOKEN_LENGTH {
        panic!("invalid bot token");
    }

    return match Discord::from_bot_token(bot_token) {
        Ok(discord) => discord,
        Err(err) => panic!("error happened: {}", err)
    }
}

fn main() {
    let cfg = Config::from_env_and_args();
    let discord = get_discord(cfg.bot_token.as_str());
}

#[cfg(test)]
mod tests {
    use crate::get_discord;

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
}
