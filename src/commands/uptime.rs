use crate::commands::command_handler::CommandHandler;
use crate::bot_core::context::RzContext;

use std::time::SystemTime;
use std::sync::Arc;

use serenity::async_trait;
use serenity::client::Context;
use serenity::prelude::{TypeMap, RwLock};
use serenity::model::channel::Message;

use chrono::DateTime;
use chrono::offset::Local;

const COMMAND_NAME: &str = "uptime";
const UPTIME_KEY: &str = "uptime";

pub struct UptimeHandler;

#[async_trait]
impl CommandHandler for UptimeHandler {
    async fn init(ctx: Arc<RwLock<TypeMap>>) {
        let now = SystemTime::now();
        let datetime: DateTime<Local> = now.into();
        RzContext::set_meta_value(ctx, UPTIME_KEY.to_string(), datetime.format("%d/%m/%Y %T").to_string()).await;
    }

    async fn registry(ctx: Arc<RwLock<TypeMap>>) {
        RzContext::registry_command(ctx, String::from(COMMAND_NAME)).await;
    }

    async fn process(ctx: &Context, msg: &Message) {
        msg.reply(ctx, format!("```Raziel start at {}```", RzContext::get_meta_value(ctx.data.clone(), UPTIME_KEY.to_string()).await)).await.unwrap();
    }
}