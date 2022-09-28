mod commands;
//Serenity crate for Discord
use serenity::model::application::command::Command;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;

#[macro_use]
//.env variables
extern crate dotenv_codegen;

//Constants
// Your Bot token
const DISCORD_TOKEN: &str = dotenv!("DISCORD_TOKEN");
// If you want to have commands specific to only a specific guild, set this as your guild_id.
const PRIVATEGUILDID: GuildId = GuildId(703332075914264606);
// Error channel, instead of logging the terminal and booting into the server, it logs errors to a private Discord.
const DISCORD_CHANNEL_ERROR: ChannelId = ChannelId(703332075914264609);

async fn on_ready(
    ctx: &Context,
    ready: &Ready,
    framework: &poise::Framework<(), serenity::Error>,
) -> Result<(), serenity::Error> {
    // To announce that the bot is online.
    println!("{} is connected!", ready.user.name);

    // This registers commands for the bot, guild commands are instantly active on specified servers
    //
    // The commands you specify here only work in your own guild!
    // This is useful if you want to control your bot from within your personal server,
    // but dont want other servers to have access to it.
    // For example sending an announcement to all servers it is located in.
    let builder = poise::builtins::create_application_commands(&framework.options().commands);
    let commands = GuildId::set_application_commands(&PRIVATEGUILDID, &ctx.http, |commands| {
        *commands = builder.clone();
        commands
    })
    .await;
    // This line runs on start-up to tell you which commands succesfully booted.
    println!(
        "I now have the following guild slash commands: \n{:#?}",
        commands
    );

    // Below we register Global commands, global commands can take some time to update on all servers the bot is active in
    //
    // Global commands are availabe in every server, including DM's.
    // We call the commands folder, the ping file and then the register function.
    let global_command1 = Command::set_global_application_commands(&ctx.http, |commands| {
        *commands = builder;
        commands
    })
    .await;
    println!(
        "I now have the following guild slash commands: \n{:#?}",
        global_command1
    );

    Ok(())
}

async fn on_error(
    error: poise::FrameworkError<'_, (), serenity::Error>,
) -> Result<(), serenity::Error> {
    match error {
        poise::FrameworkError::Command { ctx, error } => {
            DISCORD_CHANNEL_ERROR
                .say(ctx.discord(), format!("Error line 57 \n {:?}", error))
                .await?;
        }
        error => poise::builtins::on_error(error).await?,
    }
    Ok(())
}

#[allow(unused_doc_comments)]
#[tokio::main]
async fn main() {
    // Build our client.
    let client = poise::Framework::builder()
        .token(DISCORD_TOKEN)
        .intents(GatewayIntents::empty())
        .options(poise::FrameworkOptions {
            commands: vec![commands::priceinfo::info_coin()],
            on_error: |error| {
                Box::pin(async move {
                    if let Err(_why1) = on_error(error).await {
                        //not error handling this deep but if you want to you can add a println!("{:?}, _why1"); here, note that if you detach the terminal this might become funky.
                    }
                })
            },
            ..Default::default()
        })
        .user_data_setup(|ctx, ready, framework| Box::pin(on_ready(ctx, ready, framework)))
        .build()
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
