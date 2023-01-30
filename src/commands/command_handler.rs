use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;

#[async_trait]
pub trait CommandHandler {
    fn init();
    fn registry();
    async fn process(ctx: &Context, msg: &Message);
}