use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::{StandardFramework, Configuration};

mod config;
use config::Config;

mod commands;
use crate::commands::ping::*;
use crate::commands::speak::*;
use crate::commands::help::*;

#[group]
#[commands(ping, hello, introduce, help)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("Vertex is running...");
    }
}

#[tokio::main]
async fn main() {
    let token = Config::load().bot_token;

    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix("!")); // set the bot's prefix to "~"

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error starting bot...");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the bot: {:?}", why);
    }
}
