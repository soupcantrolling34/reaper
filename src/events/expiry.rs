use serenity::prelude::Context;
use tracing::error;

use crate::{mongo::structs::ActionType, Handler};

pub async fn expire_actions(ctx: Context, handler: Handler) {
    loop {
        let expired_actions = match handler.mongo.get_expired_actions().await {
            Ok(actions) => {
                actions
            },
            Err(err) => {
                error!("Error getting expired actions: {}", err);
                continue;
            }
        };
        for action in expired_actions {
            match action.action_type {
                ActionType::Mute => {
                    match handler.unmute(&ctx, action.guild_id, action.user_id, None).await {
                        Ok(_) => {},
                        Err(err) => {
                            error!("Error unmuting user: {}", err);
                        }
                    };
                },
                ActionType::Ban => {
                    match handler.unban(&ctx, action.guild_id, action.user_id, None).await {
                        Ok(_) => {},
                        Err(err) => {
                            error!("Error unbanning user: {}", err);
                        }
                    };
                },
                _ => {}
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}