use std::sync::Arc;

use serenity::async_trait;
use serenity::client::Context;
use serenity::prelude::{TypeMap, RwLock};
use serenity::model::channel::Message;

#[async_trait]
pub trait CommandHandler {
    async fn init(ctx: Arc<RwLock<TypeMap>>);
    async fn registry(ctx: Arc<RwLock<TypeMap>>);
    async fn process(ctx: &Context, msg: &Message);
}