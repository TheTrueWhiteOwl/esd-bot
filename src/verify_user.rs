use anyhow::Result;
use poise::serenity_prelude::*;
use std::{
    str::FromStr,
    time::Duration
};
use strum::IntoEnumIterator;

use crate::grade::GradeOptions;

fn grade_select_menu(id: impl Into<String>) -> CreateSelectMenu {
    let options: Vec<CreateSelectMenuOption> = GradeOptions::iter()
        .map(|variant| CreateSelectMenuOption::new(variant.to_string(), variant.to_string()).description("[placeholder]"))
        .collect();

    CreateSelectMenu::new(
        id,
        CreateSelectMenuKind::String { options },
    ).min_values(1).max_values(1)
}

const GRADE_SELECTION_TIMEOUT: Duration = Duration::from_mins(10);

pub async fn verify_user(user: UserId, ctx: &Context) -> Result<()> {
    // TODO: randomize this ofc
    let id: String = String::from("bababooey");

    let message_to_send = CreateMessage::new()
        .content("What grade are you in?")
        .components(vec![CreateActionRow::SelectMenu(grade_select_menu(&id))]);

    let message = user.direct_message(ctx, message_to_send).await?;

    if let Some(interaction) = message
        .await_component_interaction(ctx)
        .custom_ids(vec![id])
        .timeout(GRADE_SELECTION_TIMEOUT)
        .await
    {
        if let ComponentInteractionDataKind::StringSelect { values } = &interaction.data.kind {
            if values.len() != 1 {
                //smth went wrong
            }
            let grade: GradeOptions = GradeOptions::from_str(&values[0])?;
            // TODO: create response (required by discord API)
            // interaction.create_response
            // TODO: do smth
        } else {
            //smth went wrong
        }
    } else {
        user.direct_message(ctx, CreateMessage::new().content("Selection timed out! Please run the command again.")).await?;
    }
    Ok(())
}
