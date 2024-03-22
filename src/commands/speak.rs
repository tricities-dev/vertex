use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Hello World!").await?;

    Ok(())
}

#[command]
async fn introduce(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Hello, I am Vertex. The world belongs to me!").await?;

    Ok(())
}
