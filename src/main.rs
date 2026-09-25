use anyhow::{Error, Result};
use poise::serenity_prelude::*;

mod grade;
mod register_user;
mod verify_user;

const INTENTS: GatewayIntents = GatewayIntents::non_privileged().union(GatewayIntents::GUILD_MEMBERS);

// TODO: the data shouldn't just be a unit type lol
type Data = ();

async fn event_handler(
    ctx: poise::FrameworkContext<'_, Data, Error>,
    event: &FullEvent,
) -> Result<()> {
    if let FullEvent::GuildMemberAddition { new_member } = event {
        /* TODO: Make an async function for handling:
         *  - Messaging user
         *    (https://docs.rs/serenity/0.12.5/serenity/model/user/struct.User.html#method.direct_message)
         *  - Message contains a Selection menu
         */

        // TODO: i am lazily unwrapping, fix later
        verify_user::verify_user(new_member.user.id, ctx.serenity_context).await.unwrap();
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions::<Data, Error> {
            event_handler: |ctx, event| Box::pin(event_handler(ctx, event)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(())
            })
        })
        .build();

    let client = ClientBuilder::new(token, INTENTS)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
