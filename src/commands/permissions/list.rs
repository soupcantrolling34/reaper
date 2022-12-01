use serenity::{prelude::Context, model::prelude::interaction::application_command::ApplicationCommandInteraction};
use strum::IntoEnumIterator;
use crate::{commands::{structs::CommandError, utils::messages::{send_message, defer}}, mongo::structs::Permissions};

pub async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction) -> Result<(), CommandError> {
    if let Err(err) = defer(&ctx, &cmd, true).await {
        return Err(err)
    }
    let mut message_content = "The following permissions are available:\n".to_string();
    for permission in Permissions::iter() {
        if permission != Permissions::Unknown {
            message_content.push_str(&format!("`{}`\n", permission.to_string()));
        }
    }
    return send_message(&ctx, &cmd, message_content).await;
}