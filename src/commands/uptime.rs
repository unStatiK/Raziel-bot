use crate::commands::command_handler::CommandHandler;
use crate::bot_core::context::GLOBAL_CONTEXT;

use std::time::SystemTime;

use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;

use chrono::DateTime;
use chrono::offset::Local;

const COMMAND_NAME: &str = "uptime";
const UPTIME_KEY: &str = "uptime";

pub struct UptimeHandler {}

#[async_trait]
impl CommandHandler for UptimeHandler {
    fn init() {
        let now = SystemTime::now();
        let datetime: DateTime<Local> = now.into();
        GLOBAL_CONTEXT.lock().unwrap().set_meta_value(UPTIME_KEY.to_string(), datetime.format("%d/%m/%Y %T").to_string());
    }

    fn registry() {
        GLOBAL_CONTEXT.lock().unwrap().registry_command(String::from(COMMAND_NAME));
    }

    async fn process(ctx: &Context, msg: &Message) {
        msg.reply(ctx, format!("```Raziel start at {}```", GLOBAL_CONTEXT.lock().unwrap().get_meta_value(UPTIME_KEY.to_string()) )).await.unwrap();
    }
}