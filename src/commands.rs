use crate::{Context, Error};
use poise::serenity_prelude as serenity;


/// Displays your or another user's account creation date (optional - for another user)
#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Displays your or another user's avatar (optional - for another user)
#[poise::command(prefix_command, slash_command)]
pub async fn get_avatar(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = match user {
        Some(user) => user,
        None => ctx.author().clone(),
    };
    let avatar = user.avatar_url().unwrap_or_else(|| user.default_avatar_url());
    ctx.reply(avatar).await?;
    Ok(())
}

/// Add one to the count
#[poise::command(prefix_command, slash_command)]
pub async fn count(
    ctx: Context<'_>,
) -> Result<(), Error> {
    // Lock the Mutex in a block {} so the Mutex isn't locked across an await point
    let (previous_count, total_count) = {
        let mut number = ctx.data().count.lock().unwrap(); // cant dereference mutex guard otherwise count not updated
        *number += 1;
        (*number - 1, *number)
    };
    let response = format!("Previous count was **{previous_count}**. Count is now **{total_count}**.");
    ctx.say(response).await?;
    Ok(())
}

/// Get the current count
#[poise::command(prefix_command, slash_command)]
pub async fn get_count(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let count = *ctx.data().count.lock().unwrap();
    ctx.reply(format!("The current count is **{count}**.")).await?;
    Ok(())
}

/// Display the servers the application is in (owner only)
#[poise::command(prefix_command, slash_command, owners_only)]
pub async fn servers(
    ctx: Context<'_>,
) -> Result<(), Error> {
    poise::builtins::servers(ctx).await?;
    Ok(())
}

/// Show help menu (optional - help about a specific command)
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::PrettyHelpConfiguration {
        extra_text_at_bottom: "\
Type >help command for more info on a command.
You can edit your message to the bot and the bot will edit its response.",
        ..Default::default()
    };
    poise::builtins::pretty_help(ctx, command.as_deref(), config).await?;
    Ok(())
}

/// Register the application commands (owner only)
#[poise::command(prefix_command, owners_only)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}