mod commands;

use std::{collections::HashSet, env};
use poise::serenity_prelude as serenity;
use std::sync::Mutex;


struct Data {
    count: Mutex<usize>,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}


#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().expect("Failed to load .env file");
    match serenity::utils::validate_token(&env::var("DISCORD_TOKEN").expect("Expected a token in the environment")) {
        Ok(_) => println!("Token is valid"),
        Err(why) => println!("Token is invalid: {:?}", why),
    }
    let token = &env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let options = poise::FrameworkOptions {
        commands: vec![ // TODO: Easier way to add all commands
            commands::age(),
            commands::count(),
            commands::get_count(),
            commands::get_avatar(),
            commands::servers(),
            commands::register(),
            ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(">".into()),
            ..Default::default()
        },
        // The owners of the bot, which are used for the owners_only attribute
        owners: HashSet::from([serenity::UserId::new(env::var("DISCORD_OWNER_ID").expect("Expected a user id in the environment").parse::<u64>().expect("Expected a valid user id"))]),
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework | {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    count: Mutex::new(usize::default())
                })
            })
        })
        .options(options)
        .build();

    let mut client = serenity::ClientBuilder::new(&token, intents)
        .framework(framework)
        .await
        .expect("Err creating client");

    println!("Client created");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
    println!("Client started");
}