use crate::commands::command_handler::CommandHandler;
use crate::bot_core::context::RzContext;

use std::sync::Arc;

use serenity::async_trait;
use serenity::client::Context;
use serenity::prelude::{TypeMap, RwLock};
use serenity::model::channel::Message;

const COMMAND_NAME: &str = "commands";

pub struct CommandsHandler;

#[async_trait]
impl CommandHandler for CommandsHandler {
    async fn init(_ctx: Arc<RwLock<TypeMap>>) {}

    async fn registry(ctx: Arc<RwLock<TypeMap>>) {
        RzContext::registry_command(ctx, String::from(COMMAND_NAME)).await;
    }

    async fn process(ctx: &Context, msg: &Message) {
        msg.reply(ctx, format!("```Enable commands list: [{}]```", RzContext::get_commands(ctx.data.clone()).await.join(", "))).await.unwrap();
    }
}