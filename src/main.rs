#![forbid(unsafe_code)]

mod commands;
mod bot_core;

extern crate libc_alloc;
extern crate string_builder;

use commands::command_handler::CommandHandler;
use commands::whois::WhoisHandler;
use commands::version::VersionHandler;
use commands::uptime::UptimeHandler;
use commands::commands::CommandsHandler;
use commands::cur::CurrencyHandler;
use bot_core::context::RzContext;

use std::env;
use std::sync::Arc;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

use libc_alloc::LibcAlloc;
use crate::commands::foxy::FoxyHandler;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

#[group]
#[commands(whois, version, uptime, cur, commands, foxy)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    start().await;
}

async fn init_context(ctx: Arc<RwLock<TypeMap>>) {
    RzContext::init_context(ctx).await;
}

async fn registry_commands(ctx: Arc<RwLock<TypeMap>>) {
    WhoisHandler::registry(ctx.clone()).await;
    VersionHandler::registry(ctx.clone()).await;
    UptimeHandler::registry(ctx.clone()).await;
    CurrencyHandler::registry(ctx.clone()).await;
    CommandsHandler::registry(ctx.clone()).await;
    FoxyHandler::registry(ctx.clone()).await;
}

async fn init_command_system(ctx: Arc<RwLock<TypeMap>>) {
    WhoisHandler::init(ctx.clone()).await;
    VersionHandler::init(ctx.clone()).await;
    UptimeHandler::init(ctx.clone()).await;
    CurrencyHandler::init(ctx.clone()).await;
    CommandsHandler::init(ctx.clone()).await;
    FoxyHandler::init(ctx.clone()).await;
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

    init_context(client.data.clone()).await;
    registry_commands(client.data.clone()).await;
    init_command_system(client.data.clone()).await;

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

#[command]
async fn cur(ctx: &Context, msg: &Message) -> CommandResult {
    CurrencyHandler::process(ctx, msg).await;
    Ok(())
}

#[command]
async fn commands(ctx: &Context, msg: &Message) -> CommandResult {
    CommandsHandler::process(ctx, msg).await;
    Ok(())
}

#[command]
async fn foxy(ctx: &Context, msg: &Message) -> CommandResult {
    FoxyHandler::process(ctx, msg).await;
    Ok(())
}