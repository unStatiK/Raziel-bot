use crate::commands::command_handler::CommandHandler;
use crate::bot_core::context::RzContext;

use std::sync::Arc;

use serenity::async_trait;
use serenity::client::Context;
use serenity::prelude::{TypeMap, RwLock};
use serenity::framework::standard::{Args, Delimiter};
use serenity::model::channel::Message;

use reqwest::{Error, Response};

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
            let multiplier_arg = args.single::<String>().unwrap().parse::<f32>();
            if multiplier_arg.is_ok() {
                let multiplier = multiplier_arg.unwrap();
                show_rates(ctx, msg, prepare_currency_arg(src_currency).as_str(), prepare_currency_arg(target_currency).as_str(), multiplier).await;
            } else {
                error_reply(ctx, msg, ARG_MULTIPLIER_ERROR).await;
            }
        } else {
            show_rates(ctx, msg, prepare_currency_arg(src_currency).as_str(), prepare_currency_arg(target_currency).as_str(), 0.0).await;
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

async fn show_rates(ctx: &Context, msg: &Message, src: &str, target: &str, multiplier: f32) {
    msg.reply(ctx, get_rates_str(src, target, multiplier).await).await.unwrap();
}

async fn get_rates_str(src: &str, target: &str, multiplier: f32) -> String {
    let rates = get_rates(src, target).await;
    if rates.is_ok() {
        let value = rates.unwrap().parse::<f32>().unwrap();
        let calculated_value_str = if multiplier == 0.0 {
            format!("{} -> {}: {}\n", src, target, value)
        } else {
            format!("[x{}]::{} -> {}: {}\n", multiplier, src, target, value * multiplier as f32)
        };
        return calculated_value_str;
    } else {
        format!("get currencies for {} failed\n", target)
    }
}

async fn get_rates(src: &str, target: &str) -> Result<String, Error> {
    let response = get_response(src, target).await;
    match response {
        Ok(response) => {
            let body = response.text().await;
            match body {
                Ok(payload) => {
                    let rates = if payload.is_empty() { "0.0".to_string() } else { payload };
                    return Ok(rates);
                },
                Err(e) => return Err(e),
            };
        },
        Err(e) => return Err(e),
    }
}

async fn get_response(src: &str, target: &str) -> Result<Response, Error> {
    reqwest::get(format!("https://api.coingate.com/v2/rates/merchant/{}/{}", src, target)).await
}

fn prepare_currency_arg(arg: String) -> String {
    arg.to_uppercase()
}