use anyhow::Result;
use poise::serenity_prelude::*;

mod grade;
mod register_user;
mod verify_user;

// TODO: the data shouldn't just be a unit type lol
//type FrameworkContext<'a> = poise::FrameworkContext<'a, (), Error>;

async fn event_handler(
    ctx: &Context,
    event: &FullEvent,
) -> Result<()> {
    if let FullEvent::GuildMemberAddition { new_member } = event {
        /* TODO: Make an async function for handling:
         *  - Messaging user
         *    (https://docs.rs/serenity/0.12.5/serenity/model/user/struct.User.html#method.direct_message)
         *  - Message contains a Selection menu
         */
        verify_user::verify_user(new_member.user.id, ctx);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    // TODO: important stuff like initializing the whole connection with discord??
}
