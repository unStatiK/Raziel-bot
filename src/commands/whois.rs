extern crate whois;

use crate::commands::command_handler::CommandHandler;
use crate::bot_core::db::RzDb;

use std::thread;

use serenity::async_trait;
use serenity::client::Context;
use serenity::framework::standard::{Args, Delimiter};
use serenity::model::channel::Message;

use whois::WhoIs;
use rustc_serialize::json::Json;
use string_builder::Builder;

pub struct WhoisHandler {}

const HELP_ARGUMENT: &str = "help";
const SAVE_ARGUMENT: &str = "save";
const DEL_ARGUMENT: &str = "del";
const LIST_ARGUMENT: &str = "list";
const SHOW_ARGUMENT: &str = "show";
const ARG_DOMAIN_ERROR: &str = "error: need provide domain name";

#[async_trait]
impl CommandHandler for WhoisHandler {
    fn init() {
        let conn = RzDb::get_connection();
        let query = "CREATE TABLE IF NOT EXISTS domains (
            id   INTEGER PRIMARY KEY,
            domain TEXT NOT NULL UNIQUE
        )";
        RzDb::tx_execute(&conn, query);
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
    let conn = RzDb::get_connection();
    RzDb::tx_execute(&conn, format!("INSERT INTO domains (domain) VALUES ('{}')", domain).as_str());
    msg.reply(ctx, format!("domain {} saved!", domain)).await.unwrap();
}

async fn delete_domain(ctx: &Context, msg: &Message, domain: &str) {
    let conn = RzDb::get_connection();
    RzDb::tx_execute(&conn, format!("DELETE FROM domains where domain = '{}'", domain).as_str());
    msg.reply(ctx, format!("domain {} delete!", domain)).await.unwrap();
}

async fn show_list_domains(ctx: &Context, msg: &Message) {
    let mut builder = Builder::default();
    let handler = get_all_saved_domains();
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
    msg.reply(ctx, get_expire_date_str(&String::from(domain))).await.unwrap();
}

async fn show_all_expire_date(ctx: &Context, msg: &Message) {
    let mut builder = Builder::default();
    let handler = get_all_saved_domains();
    for domain in handler.iter() {
        builder.append(get_expire_date_str(domain));
    }
    let content = builder.string().unwrap();
    if !content.is_empty() {
        msg.reply(ctx, content).await.unwrap();
    }
}

fn get_all_saved_domains() -> Vec<String> {
    return thread::spawn(|| {
        let mut domains = Vec::new();
        let conn = RzDb::get_connection();
        let mut stmt = conn.prepare("SELECT domain FROM domains").unwrap();
        let rows = stmt.query_map([], |row| {
            row.get::<usize, String>(0)
        }).unwrap();
        for domain_name in rows {
            let dom = domain_name.unwrap();
            domains.push(dom);
        }
        return domains;
    }).join().unwrap();
}

fn get_expire_date_str(domain: &String) -> String {
    return format!("{} expire -> {}\n", domain, get_expire_date(domain));
}

fn get_expire_date(domain: &str) -> String {
    let whois_response = WhoIs::new(domain.to_owned()).lookup();
    let whois_json = &Json::from_str(&whois_response.unwrap()).unwrap();
    let whois = whois_json.as_object().unwrap();
    let mut expire = whois.get("   Registry Expiry Date");
    if expire != None {
        return expire.unwrap().to_string();
    }
    expire = whois.get("free-date");
    if expire != None {
        return expire.unwrap().to_string();
    }
    return "".to_string();
}