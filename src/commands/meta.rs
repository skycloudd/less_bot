use serenity::framework::standard::macros::{command, help};
use serenity::framework::standard::{
    help_commands, Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections;

#[command]
#[description = "This command is literally useless unless youre the bot dev"]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "pong!").await?;

    Ok(())
}

#[command]
#[description = "Some info about the bot"]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let output = format!(
        "`{} v{}`\n\
        Powered by Rust and Serenity 🦀\n\
        Source code available on GitHub: <{}>
        ",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        "https://github.com/Skycloudd/less_bot"
    );

    msg.channel_id.say(&ctx.http, &output).await?;

    Ok(())
}

#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: collections::HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
