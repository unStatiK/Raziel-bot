#![forbid(unsafe_code)]

extern crate libc_alloc;
extern crate rustc_serialize;
extern crate string_builder;

mod commands;
mod bot_core;

use std::env;

use commands::command_handler::CommandHandler;
use commands::whois::WhoisHandler;
use commands::version::VersionHandler;
use commands::uptime::UptimeHandler;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

use libc_alloc::LibcAlloc;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

#[group]
#[commands(whois, version, uptime)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    init_command_system();
    start().await;
}

fn init_command_system() {
    WhoisHandler::init();
    VersionHandler::init();
    UptimeHandler::init();
}

async fn start() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn whois(ctx: &Context, msg: &Message) -> CommandResult {
    WhoisHandler::process(ctx, msg).await;
    Ok(())
}

#[command]
async fn version(ctx: &Context, msg: &Message) -> CommandResult {
    VersionHandler::process(ctx, msg).await;
    Ok(())
}

#[command]
async fn uptime(ctx: &Context, msg: &Message) -> CommandResult {
    UptimeHandler::process(ctx, msg).await;
    Ok(())
}