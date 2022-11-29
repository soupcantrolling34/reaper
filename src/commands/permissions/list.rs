use serenity::{prelude::Context, model::prelude::interaction::application_command::ApplicationCommandInteraction};
use strum::IntoEnumIterator;
use crate::{commands::{structs::CommandError, utils::messages::send_message}, mongo::structs::Permissions};

pub async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction) -> Result<(), CommandError> {
    let mut message_content = "The following permissions are available:\n".to_string();
    for permission in Permissions::iter() {
        if permission != Permissions::Unknown {
            message_content.push_str(&format!("`{}`\n", permission.to_string()));
        }
    }
    return send_message(&ctx, &cmd, message_content, Some(true)).await;
}