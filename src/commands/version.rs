extern crate os_info;

use std::sync::Arc;

use crate::commands::command_handler::CommandHandler;
use crate::bot_core::context::RzContext;
use crate::bot_core::constants::RAZIEL_VERSION;

use serenity::async_trait;
use serenity::client::Context;
use serenity::prelude::{TypeMap, RwLock};
use serenity::model::channel::Message;

const COMMAND_NAME: &str = "version";

pub struct VersionHandler;

#[async_trait]
impl CommandHandler for VersionHandler {
    async fn init(_ctx: Arc<RwLock<TypeMap>>) {}

    async fn registry(ctx: Arc<RwLock<TypeMap>>) {
        RzContext::registry_command(ctx, String::from(COMMAND_NAME)).await;
    }

    async fn process(ctx: &Context, msg: &Message) {
        msg.reply(ctx, format!("```core version: {} [{}]```", RAZIEL_VERSION, os_info::get())).await.unwrap();
    }
}