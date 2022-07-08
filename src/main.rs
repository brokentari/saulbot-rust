use rand::Rng;
use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tiberius::{error::Error, AuthMethod, Client, ColumnData, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

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
#[commands(ping, gamble, roll, personalize, message_count)]
struct General;

struct Handler;

struct MessageCount;

impl TypeMapKey for MessageCount {
    type Value = Arc<AtomicUsize>;
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let count = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<MessageCount>()
                .expect("expected msgcount in typemap")
                .clone()
        };

        count.fetch_add(1, Ordering::SeqCst);

        if msg.mentions.len() > 0 {
            // open the message.json file
            let mut message_file = File::open("./messages.json").unwrap();
            // read the message.json file
            let mut data = String::new();
            message_file
                .read_to_string(&mut data)
                .expect("Unable to read message.json");

            // deserialize the message.json file
            let list_of_messages: PersonalizedMessages =
                serde_json::from_str(&data).expect("Unable to deserialize message.json");

            for user in msg.mentions.iter() {
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, &list_of_messages.messages[0].message)
                    .await
                {
                    println!("error sending message: {:?}", why);
                }
                let _ = msg
                    .channel_id
                    .say(&ctx.http, format!("You mentioned {}!", user.name))
                    .await;
            }
        }
    }
}

async fn init_database() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();

    config.host("brokentari.database.windows.net");
    config.port(1433);
    config.database("messages");
    config.authentication(AuthMethod::sql_server("saul", "Bobby67Six"));

    config.trust_cert();

    let tcp_result = TcpStream::connect(config.get_addr()).await;

    match tcp_result {
        Ok(stream) => {
            stream.set_nodelay(true).unwrap();
            let mut client = match Client::connect(config, stream.compat_write()).await {
                Ok(client) => client,
                Err(Error::Routing { host, port }) => {
                    let mut config = Config::new();

                    config.host(&host);
                    config.port(port);
                    config.authentication(AuthMethod::sql_server("saul", "Bobby67Six"));

                    let tcp = TcpStream::connect(config.get_addr()).await.unwrap();
                    tcp.set_nodelay(true).unwrap();

                    Client::connect(config, tcp.compat_write()).await.unwrap()
                }
                Err(e) => Err(e).unwrap(),
            };

            let res = client
                .query(
                    "SELECT TOP (@P1) [MessageId], [DiscordId], [MessageText] FROM message WHERE [DiscordId] = @P2",
                    &[&1i32, &"12345"],
                )
                .await
                .unwrap()
                .into_row()
                .await
                .unwrap()
                .unwrap();

            res.get::<i32, &str>("MessageId").unwrap();

            for column in res {
                match column {
                    ColumnData::I32(result) => {
                        println!("{}", result.unwrap())
                    }
                    ColumnData::String(result) => {
                        println!("{}", result.unwrap())
                    }
                    _ => {}
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    init_database().await.unwrap();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("fucking "))
        .group(&GENERAL_GROUP);

    let token =
        String::from("OTkwNjgzNjg0Mjc1MzcyMDYy.GML8cl.qm6y81fkpHTOGYqnVdjnRXlcaKdyfNQqessGKY");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<MessageCount>(Arc::new(AtomicUsize::new(0)));
    }

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
    let mut message_file = File::open("./messages.json").unwrap();
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
        let mut message_file = File::create("./messages.json").unwrap();
        message_file
            .write_all(serialized_list_of_messages.as_bytes())
            .expect("Unable to  write to message.json");

        msg.reply(ctx, "message set sucessfully!").await?;
    }

    Ok(())
}

#[command]
async fn message_count(ctx: &Context, msg: &Message) -> CommandResult {
    let raw_count = {
        let data_read = ctx.data.read().await;

        data_read
            .get::<MessageCount>()
            .expect("expected messagecount in typemap")
            .clone()
    };

    let count = raw_count.load(Ordering::Relaxed);

    if count == 1 {
        msg.reply(
            ctx,
            "you are the first one to send a message while this bot is running!",
        )
        .await?;
    } else {
        msg.reply(ctx, format!("you have sent {} messages", count))
            .await?;
    }

    Ok(())
}
