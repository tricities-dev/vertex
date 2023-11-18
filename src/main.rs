use discord::Discord;

extern crate discord;

fn main() {
    println!("Hello, world!");
    let discord = Discord::from_bot_token("my_token");
}
