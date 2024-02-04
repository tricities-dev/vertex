use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;


#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "List of commands\n1. !help - Get a list of commands.\n2. !introduce - Introduces the bot.\n3. !ping - Ping the bot!\n4. !hello - Hello World from the bot.\n").await?;

    Ok(())
}
