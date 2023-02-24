use crate::commands::command_handler::CommandHandler;
use crate::bot_core::context::RzContext;

use std::sync::Arc;

use serenity::async_trait;
use serenity::client::Context;
use serenity::prelude::{TypeMap, RwLock};
use serenity::model::channel::Message;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use rand::Rng;

const COMMAND_NAME: &str = "foxy";

#[derive(Serialize, Deserialize)]
struct RandomApi {
    link: String,
}

#[derive(Serialize, Deserialize)]
struct RandomFox {
    image: String,
}

pub struct FoxyHandler;

#[async_trait]
impl CommandHandler for FoxyHandler {
    async fn init(_ctx: Arc<RwLock<TypeMap>>) {}

    async fn registry(ctx: Arc<RwLock<TypeMap>>) {
        RzContext::registry_command(ctx, String::from(COMMAND_NAME)).await;
    }

    async fn process(ctx: &Context, msg: &Message) {
        let res = rand::thread_rng().gen_range(1..3);
        match res {
            1 => process_reply_with_link(ctx, msg, get_link_from_random_api().await).await,
            2 => process_reply_with_link(ctx, msg, get_link_from_random_fox().await).await,
            _ => ()
        }
    }
}

async fn process_reply_with_link(ctx: &Context, msg: &Message, link: String) {
    msg.reply(ctx, format!("This is Foxy!\n{}", link)).await.unwrap();
}

async fn get_link_from_random_api() -> String {
    let url = "https://some-random-api.ml/img/fox";
    return extract_link_from_random_api(get_foxy_content(url).await.as_str()).unwrap();
}

async fn get_link_from_random_fox() -> String {
    let url = "https://randomfox.ca/floof/";
    return extract_link_from_random_fox(get_foxy_content(url).await.as_str()).unwrap();
}

async fn get_foxy_content(url: &str) -> String {
    let response = reqwest::get(url).await.unwrap();
    response.text().await.unwrap()
}

fn extract_link_from_random_api(data: &str) -> Result<String> {
    let p: RandomApi = serde_json::from_str(data)?;
    Ok(p.link)
}

fn extract_link_from_random_fox(data: &str) -> Result<String> {
    let p: RandomFox = serde_json::from_str(data)?;
    Ok(p.image)
}