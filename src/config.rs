use dotenv::dotenv;
use std::env;

pub struct Config {
    pub bot_token: String,
}

impl Config {
    fn build(bot_token: String) -> Self {
        Config {
            bot_token: bot_token,
        }
    }

    pub fn load() -> Self {
        dotenv().ok();

        println!("Config Loaded Successfully...");

        Config::build(env::var("BOT_TOKEN").expect("No Bot Token Found."))
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

    const BOT_TOKEN_LENGTH: usize = 72;

    #[test]
    fn verify_token() {
        let token = Config::load().bot_token;

        if token.len() != BOT_TOKEN_LENGTH {
            panic!(
                "Bot token length != 72. Current Bot Length: {}",
                token.len()
            );
        }
    }
}
