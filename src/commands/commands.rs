use crate::commands::command_handler::CommandHandler;
use crate::bot_core::context::GLOBAL_CONTEXT;

use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;

const COMMAND_NAME: &str = "commands";

pub struct CommandsHandler {}

#[async_trait]
impl CommandHandler for CommandsHandler {
    fn init() {}

    fn registry() {
        GLOBAL_CONTEXT.lock().unwrap().registry_command(String::from(COMMAND_NAME));
    }

    async fn process(ctx: &Context, msg: &Message) {
        msg.reply(ctx, format!("```Enable commands list: [{}]```", GLOBAL_CONTEXT.lock().unwrap().get_commands().join(", "))).await.unwrap();
    }
}