use serenity::{prelude::Context, model::prelude::interaction::{application_command::ApplicationCommandInteraction, InteractionResponseType}};

use crate::commands::structs::CommandError;

pub async fn send_message(ctx: &Context, cmd: &ApplicationCommandInteraction, content: String, ephermal: Option<bool>) -> Result<(), CommandError> {
    let mut is_ephermal = false;
    if let Some(ephermal) = ephermal {
        is_ephermal = ephermal;
    }
    match cmd.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| {
                message
                    .content(content)
                    .ephemeral(is_ephermal)
                    .allowed_mentions(|allowed_mentions| {
                        allowed_mentions.empty_parse()
                    })
            })
    }).await {
        Ok(_) => Ok(()),
        Err(_) => {
            return Err(CommandError {
                message: "Could not send message".to_string(),
                command_error: None
            })
        }
    }
}