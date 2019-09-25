use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    Args,
    macros::{
        command,
        group
    }
};

use destiny::{ parse_dice_string, roll_complexity , DiceDistribution };

group!({
    name: "general",
    options: {},
    commands: [roll, dist],
});

use std::env;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
fn roll(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {

    let complexity = roll_complexity(args.rest());

    let content= format!("rolls a {}", parse_dice_string(args.rest()));

    msg.reply(ctx, content)?;

    Ok(())
}

#[command]
fn dist(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {

    let complexity = roll_complexity(args.rest());

    let mut content = String::from("");

    if complexity <= 1000 {
        let dd = DiceDistribution::new(args.rest());
        content = format!("\n```{}```", dd.table().to_string());
    } else {
        content = format!("There are {} possibilities for that roll. The maximum is 1000", complexity);
    }
    msg.reply(ctx, content)?;

    Ok(())
}
