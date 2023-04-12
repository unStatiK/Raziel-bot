use crate::commands::command_handler::CommandHandler;
use crate::bot_core::context::RzContext;
use crate::bot_core::db::RzDb;

use std::sync::Arc;
use std::time::UNIX_EPOCH;

use serenity::async_trait;
use serenity::client::Context;
use serenity::prelude::{TypeMap, RwLock};
use serenity::framework::standard::{Args, Delimiter};
use serenity::model::channel::Message;

use sqlx::Row;
use string_builder::Builder;
use reqwest::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct WhoisHandler;

const COMMAND_NAME: &str = "whois";
const HELP_ARGUMENT: &str = "help";
const SAVE_ARGUMENT: &str = "save";
const DEL_ARGUMENT: &str = "del";
const LIST_ARGUMENT: &str = "list";
const SHOW_ARGUMENT: &str = "show";
const ARG_DOMAIN_ERROR: &str = "error: need provide domain name";

#[derive(Serialize, Deserialize)]
struct WhoisApi {
    expires: u64
}

#[async_trait]
impl CommandHandler for WhoisHandler {
    async fn init(_ctx: Arc<RwLock<TypeMap>>) {
        let conn = RzDb::get_connection().await;
        let mut init_fail = false;
        if conn.is_ok() {
            let query = r#" CREATE TABLE IF NOT EXISTS domains (
                        id     INTEGER PRIMARY KEY,
                        domain TEXT NOT NULL UNIQUE
                    )"#;
            let init_result = RzDb::tx_execute(&conn.unwrap(), query).await;
            if !init_result {
                init_fail = true;
            }
        } else {
            init_fail = true;
        }

        if init_fail {
            panic!("Init bot failed!");
        }
    }

    async fn registry(ctx: Arc<RwLock<TypeMap>>) {
        RzContext::registry_command(ctx, String::from(COMMAND_NAME)).await;
    }

    async fn process(ctx: &Context, msg: &Message) {
        let mut args = Args::new(msg.content.as_str(), &[Delimiter::Single(' ')]);
        if args.len() == 1 {
            show_all_expire_date(ctx, msg).await;
        }

        if args.len() > 1 {
            //skip first arg '!whois'
            args.advance();
            let argument = args.current().unwrap();
            match argument {
                HELP_ARGUMENT => print_help(ctx, msg).await,
                SAVE_ARGUMENT => {
                    if args.len() > 2 {
                        save_domain(ctx, msg, args.advance().current().unwrap()).await;
                    } else {
                        error_reply(ctx, msg, ARG_DOMAIN_ERROR).await;
                    }
                }
                DEL_ARGUMENT => {
                    if args.len() > 2 {
                        delete_domain(ctx, msg, args.advance().current().unwrap()).await;
                    } else {
                        error_reply(ctx, msg, ARG_DOMAIN_ERROR).await;
                    }
                }
                LIST_ARGUMENT => show_list_domains(ctx, msg).await,
                SHOW_ARGUMENT => {
                    if args.len() > 2 {
                        show_expire_date(ctx, msg, args.advance().current().unwrap()).await;
                    } else {
                        error_reply(ctx, msg, ARG_DOMAIN_ERROR).await;
                    }
                }
                _ => {}
            }
        }
    }
}

async fn error_reply(ctx: &Context, msg: &Message, error: &str) {
    msg.reply(ctx, error).await.unwrap();
}

async fn save_domain(ctx: &Context, msg: &Message, domain: &str) {
    let conn = RzDb::get_connection().await;
    let mut save_result = false;
    if conn.is_ok() {
        save_result = RzDb::tx_execute(&conn.unwrap(), format!("INSERT INTO domains (domain) VALUES ('{}')", domain).as_str()).await;
    }
    if save_result {
        msg.reply(ctx, format!("domain {} saved!", domain)).await.unwrap();
    } else {
        msg.reply(ctx, "domain save failed!").await.unwrap();
    }
}

async fn delete_domain(ctx: &Context, msg: &Message, domain: &str) {
    let conn = RzDb::get_connection().await;
    let mut del_result = false;
    if conn.is_ok() {
        del_result = RzDb::tx_execute(&conn.unwrap(), format!("DELETE FROM domains where domain = '{}'", domain).as_str()).await;
    }
    if del_result {
        msg.reply(ctx, format!("domain {} delete!", domain)).await.unwrap();
    } else {
        msg.reply(ctx, "domain delete failed!").await.unwrap();
    }
}

async fn show_list_domains(ctx: &Context, msg: &Message) {
    let mut builder = Builder::default();
    let handler = get_all_saved_domains().await;
    builder.append("list domains:\n");
    for domain in handler.iter() {
        builder.append(format!("{}\n", domain.as_str()));
    }
    msg.reply(ctx, builder.string().unwrap()).await.unwrap();
}

async fn print_help(ctx: &Context, msg: &Message) {
    msg.reply(ctx, "```\
    !whois help - show this help\n\
    !whois - show whois for all saved domain\n\
    !whois show domain - show whois for domain\n\
    !whois save domain - save domain for show whois\n\
    !whois del domain - remove domain from show whois list\n\
    !whois list - show all saved domain```").await.unwrap();
}

async fn show_expire_date(ctx: &Context, msg: &Message, domain: &str) {
    msg.reply(ctx, get_expire_date_str(&String::from(domain)).await).await.unwrap();
}

async fn show_all_expire_date(ctx: &Context, msg: &Message) {
    let mut builder = Builder::default();
    let handler = get_all_saved_domains().await;
    for domain in handler.iter() {
        builder.append(get_expire_date_str(domain).await);
    }
    let content = builder.string().unwrap();
    if !content.is_empty() {
        msg.reply(ctx, content).await.unwrap();
    }
}

async fn get_all_saved_domains() -> Vec<String> {
    let conn = RzDb::get_connection().await;
    let mut domains = Vec::new();
    if conn.is_ok() {
        let rows = sqlx::query("SELECT domain FROM domains").fetch_all(&conn.unwrap()).await;
        if rows.is_ok() {
            for row in rows.unwrap().iter() {
                let domain = row.get(0);
                domains.push(domain);
            }
        }
    }
    domains
}

async fn get_expire_date_str(domain: &String) -> String {
    format!("{} expire -> {}\n", domain, get_expire_date(domain).await)
}

async fn get_expire_date(domain: &str) -> String {
    let whois_response = get_whois_content(domain).await;
    match whois_response {
        Ok(response) => {
            let whois = serde_json::from_str(&response);
            if whois.is_ok() {
                let whois_json: WhoisApi = whois.unwrap();
                let unix_time = whois_json.expires;
                let timestamp = UNIX_EPOCH + std::time::Duration::from_secs(unix_time as u64);
                return DateTime::<Utc>::from(timestamp).format("%Y-%m-%d %H:%M:%S").to_string();
            }
            "".to_string()
        },
        Err(_e) => return String::from("undefined"),
    }
}

async fn get_whois_content(domain: &str) -> Result<String, Error> {
    let url = format!("http://api.whois.vu/?q={}&clean", domain);
    let response = reqwest::get(url).await;
    match response {
        Ok(response) => {
            let body = response.text().await;
            match body {
                Ok(payload) => Ok(payload),
                Err(e) => return Err(e),
            }
        },
        Err(e) => return Err(e),
    }
}