use serenity::async_trait;
use serenity::model::gateway::GatewayIntents;
use serenity::model::guild::Member;
use serenity::prelude::*;
use serenity::model::event::GuildMemberAdd;
use serenity::model::event::GuildMemberRemove;
use serenity::model::channel::ChannelType;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, _member: GuildMemberAdd) {
        // Get the channel to send the message
        if let Some(guild_id) = _member.guild_id.to_channel(&ctx).await.ok() {
            if let Some(channel) = guild_id
                .id
                .channels(&ctx)
                .await
                .ok()
                .and_then(|channels| channels.into_iter().find(|c| c.kind == ChannelType::Text))
            {
                if let Err(why) = channel.id.say(&ctx.http, format!("Welcome to the server mate, {}!", _member.user.name)).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }

    async fn guild_member_removal(&self, ctx: Context, _member: GuildMemberRemove) {
        // Get the channel to send the message
        if let Some(guild_id) = _member.guild_id.to_channel(&ctx).await.ok() {
            if let Some(channel) = guild_id
                .id
                .channels(&ctx)
                .await
                .ok()
                .and_then(|channels| channels.into_iter().find(|c| c.kind == ChannelType::Text))
            {
                if let Err(why) = channel.id.say(&ctx.http, format!("Goodbye mate, {}!", _member.user.name)).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = "YOUR_BOT_TOKEN_HERE";

    let intents = GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILD_MESSAGES;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
