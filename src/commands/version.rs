extern crate os_info;

use crate::commands::command_handler::CommandHandler;
use crate::bot_core::constants::RAZIEL_VERSION;

use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;

pub struct VersionHandler {}

#[async_trait]
impl CommandHandler for VersionHandler {
    fn init() {}

    async fn process(ctx: &Context, msg: &Message) {
        msg.reply(ctx, format!("```core version: {} [{}]```", RAZIEL_VERSION, os_info::get())).await.unwrap();
    }
}