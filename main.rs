mod commands;
//Serenity crate for Discord
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;

#[macro_use]
//.env variables
extern crate dotenv_codegen;

//Constants
/// Your Bot token
const DISCORD_TOKEN: &str = dotenv!("DISCORD_TOKEN");
/// If you want to have commands specific to only a specific guild, set this as your guild_id.
const PRIVATEGUILDID: GuildId = GuildId(1014660478351454299);
/// Error channel, instead of logging the terminal and booting into the server, it logs errors to a private Discord.
const DISCORD_CHANNEL_ERROR: ChannelId = ChannelId(1017194118742556703);

struct Handler;

/// This file uses triple slashes to comment the code,
/// and double slashes for temporarily unused code.
#[allow(unused_doc_comments)]
#[allow(unused_attributes)]
#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            //println!("Received command interaction: {:#?}", command);

            /// This receives the response content from commands.
            /// It uses 'content' to post it in the discord a few lines below.
            let content = match command.data.name.as_str() {
                /// call the function that runs the command.
                /// in this case we call the folder commands, and then call the file ping followed by the function 'run'.
                "info_coin" => commands::priceinfo::run(&command.data.options).await,

                // Here we handle every command that is not implemented, or does not return a value.
                _ => "An error :(".to_string(),
            };

            /// Here 'content' gets used to respond to the user
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                if let Err(_why1) = DISCORD_CHANNEL_ERROR
                    .say(&ctx.http, format!("Error line 57 \n {:?}", why))
                    .await
                {
                    //not error handling this deep but if you want to you can add a println!("{:?}, _why1"); here, note that if you detach the terminal this might become funky.
                }
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        /// To announce that the bot is online.
        println!("{} is connected!", ready.user.name);

        /// This registers commands for the bot, guild commands are instantly active on specified servers
        ///
        /// The commands you specify here only work in your own guild!
        /// This is useful if you want to control your bot from within your personal server,
        /// but dont want other servers to have access to it.
        /// For example sending an announcement to all servers it is located in.
        let commands = GuildId::set_application_commands(&PRIVATEGUILDID, &ctx.http, |commands| {
            commands.create_application_command(|command| commands::priceinfo::register(command))
        })
        .await;
        /// This line runs on start-up to tell you which commands succesfully booted.
        println!(
            "I now have the following guild slash commands: \n{:#?}",
            commands
        );

        /// Below we register Global commands, global commands can take some time to update on all servers the bot is active in
        ///
        /// Global commands are availabe in every server, including DM's.
        /// We call the commands folder, the ping file and then the register function.
        let global_command1 = Command::create_global_application_command(&ctx.http, |command| {
            commands::priceinfo::register(command)
        })
        .await;
        println!(
            "I now have the following guild slash commands: \n{:#?}",
            global_command1
        );
    }
}

#[allow(unused_doc_comments)]
#[tokio::main]
async fn main() {
    /// Build our client.
    let mut client = Client::builder(DISCORD_TOKEN, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
