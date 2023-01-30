extern crate os_info;

use crate::commands::command_handler::CommandHandler;
use crate::bot_core::constants::RAZIEL_VERSION;
use crate::bot_core::context::GLOBAL_CONTEXT;

use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;

const COMMAND_NAME: &str = "version";

pub struct VersionHandler {}

#[async_trait]
impl CommandHandler for VersionHandler {
    fn init() {}

    fn registry() {
        GLOBAL_CONTEXT.lock().unwrap().registry_command(String::from(COMMAND_NAME));
    }

    async fn process(ctx: &Context, msg: &Message) {
        msg.reply(ctx, format!("```core version: {} [{}]```", RAZIEL_VERSION, os_info::get())).await.unwrap();
    }
}