use rand::Rng;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PersonalizedMessage {
    user_id: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PersonalizedMessages {
    messages: Vec<PersonalizedMessage>,
}

#[group]
#[commands(ping, gamble, roll, personalize)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let token =
        String::from("OTkwNjgzNjg0Mjc1MzcyMDYy.GML8cl.qm6y81fkpHTOGYqnVdjnRXlcaKdyfNQqessGKY");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("error creating client");

    if let Err(why) = client.start().await {
        println!("an error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn gamble(ctx: &Context, msg: &Message) -> CommandResult {
    // generate a random number between 1 and 5
    let random_number: u32 = rand::thread_rng().gen_range(1..6);
    println!("{}", random_number);

    if msg.author.id.to_string() == "141255836914679808" {
        msg.reply(ctx, "You win two lotteries!").await?;
    } else {
        msg.reply(ctx, "You lose! :(").await?;
    }

    Ok(())
}

#[command]
async fn roll(ctx: &Context, msg: &Message) -> CommandResult {
    // generate a random number between 1 and 6
    let random_number: u32 = rand::thread_rng().gen_range(1..7);
    println!("{}", random_number);

    if msg.author.id.to_string() == "141255836914679808" {
        msg.reply(ctx, "You rolled an infinite").await?;
    } else {
        msg.reply(ctx, format!("You rolled a {}!", random_number))
            .await?;
    }

    Ok(())
}

#[command]
async fn personalize(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // open the message.json file
    let mut message_file = File::open("messages.json").unwrap();
    // read the message.json file
    let mut data = String::new();
    message_file
        .read_to_string(&mut data)
        .expect("Unable to read message.json");

    // deserialize the message.json file
    let mut list_of_messages: PersonalizedMessages =
        serde_json::from_str(&data).expect("Unable to deserialize message.json");

    let possible_entry = list_of_messages
        .messages
        .iter_mut()
        .find(|entry| entry.user_id == msg.author.id.to_string());

    if args.current().unwrap() == "view" {
        match possible_entry {
            Some(entry) => {
                msg.reply(
                    ctx,
                    format!("Your personalized message is: {}", entry.message),
                )
                .await?;
            }
            None => {
                msg.reply(
                    ctx,
                    "You have no personalized message! Set one with 'personalize set ...'",
                )
                .await?;
            }
        }
    } else if args.single::<String>().unwrap() == "set" {
        // get the user id
        let user_id = msg.author.id.to_string();

        // get the message
        let message = String::from(args.rest());

        match possible_entry {
            Some(entry) => entry.message = message,
            None => {
                // create a new personalized message
                let new_message = PersonalizedMessage { user_id, message };
                // add the new personalized message to the list of messages
                list_of_messages.messages.push(new_message);
            }
        }

        // serialize the list of messages
        let serialized_list_of_messages =
            serde_json::to_string(&list_of_messages).expect(r#"Unable to serialize"#);
        // write the serialized list of messages to the message.json file
        let mut message_file = File::create("messages.json").unwrap();
        message_file
            .write_all(serialized_list_of_messages.as_bytes())
            .expect("Unable to write to message.json");

        msg.reply(ctx, "message set sucessfully!").await?;
    }

    Ok(())
}
