extern crate whois;

use crate::commands::command_handler::CommandHandler;
use crate::bot_core::context::RzContext;

use std::sync::Arc;

use serenity::async_trait;
use serenity::client::Context;
use serenity::prelude::{TypeMap, RwLock};
use serenity::framework::standard::{Args, Delimiter};
use serenity::model::channel::Message;

pub struct CurrencyHandler;

const COMMAND_NAME: &str = "cur";
const HELP_ARGUMENT: &str = "help";
const ARG_CURRENCIES_ERROR: &str = "error: need provide currencies in format !cur EUR RUB";
const ARG_MULTIPLIER_ERROR: &str = "error: need provide positive number in multiplier argument";

#[async_trait]
impl CommandHandler for CurrencyHandler {
    async fn init(_ctx: Arc<RwLock<TypeMap>>) {}

    async fn registry(ctx: Arc<RwLock<TypeMap>>) {
        RzContext::registry_command(ctx, String::from(COMMAND_NAME)).await;
    }

    async fn process(ctx: &Context, msg: &Message) {
        let mut args = Args::new(msg.content.as_str(), &[Delimiter::Single(' ')]);
        let args_len = args.len();
        if args_len < 2 {
            error_reply(ctx, msg, ARG_CURRENCIES_ERROR).await;
            return;
        }

        //skip first arg '!cur'
        args.advance();
        if args_len == 2 {
            let argument = args.current().unwrap();
            match argument {
                HELP_ARGUMENT => print_help(ctx, msg).await,
                _ => {}
            }
            return;
        }

        let src_currency = args.single::<String>().unwrap();
        let target_currency = args.single::<String>().unwrap();
        if args_len >= 4 {
            let multiplier_arg = args.single::<String>().unwrap().parse::<u32>();
            if multiplier_arg.is_ok() {
                let multiplier = multiplier_arg.unwrap();
                show_rates(ctx, msg, prepare_currency_arg(src_currency).as_str(), prepare_currency_arg(target_currency).as_str(), multiplier).await;
            } else {
                error_reply(ctx, msg, ARG_MULTIPLIER_ERROR).await;
            }
        } else {
            show_rates(ctx, msg, prepare_currency_arg(src_currency).as_str(), prepare_currency_arg(target_currency).as_str(), 0).await;
        }
    }
}

async fn error_reply(ctx: &Context, msg: &Message, error: &str) {
    msg.reply(ctx, error).await.unwrap();
}

async fn print_help(ctx: &Context, msg: &Message) {
    msg.reply(ctx, "```\
    !cur help - show this help\n\
    !cur EUR RUB - show rates currencies\n\
    !cur EUR RUB 10 - show x10 rates currencies```").await.unwrap();
}

async fn show_rates(ctx: &Context, msg: &Message, src: &str, target: &str, multiplier: u32) {
    msg.reply(ctx, get_rates_str(src, target, multiplier).await).await.unwrap();
}

async fn get_rates_str(src: &str, target: &str, multiplier: u32) -> String {
    let value = get_rates(src, target).await.parse::<f32>().unwrap();
    let calculated_value_str = if multiplier == 0 {
        format!("{} -> {}: {}\n", src, target, value)
    } else {
        format!("[x{}]::{} -> {}: {}\n", multiplier, src, target, value * multiplier as f32)
    };
    calculated_value_str
}

async fn get_rates(src: &str, target: &str) -> String {
    let response = reqwest::get(format!("https://api.coingate.com/v2/rates/merchant/{}/{}", src, target)).await.unwrap();
    let body = response.text().await.unwrap();
    let rates = if body.is_empty() { "0.0".to_string() } else { body };
    rates
}

fn prepare_currency_arg(arg: String) -> String {
    arg.to_uppercase()
}