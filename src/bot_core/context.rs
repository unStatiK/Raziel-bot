use std::{sync::Arc, collections::HashMap};

use serenity::prelude::{TypeMap, TypeMapKey, RwLock};

pub struct RzMeta;

impl TypeMapKey for RzMeta {
    type Value = Arc<RwLock<HashMap<String, String>>>;
}

pub struct RzCommands;

impl TypeMapKey for RzCommands {
    type Value = Arc<RwLock<Vec<String>>>;
}

pub struct RzContext;

impl RzContext {
    pub async fn init_context(ctx: Arc<RwLock<TypeMap>>) {
        let mut data = ctx.write().await;
        data.insert::<RzCommands>(Arc::new(RwLock::new(Vec::new())));
        data.insert::<RzMeta>(Arc::new(RwLock::new(HashMap::new())));
    }

    pub async fn get_meta_value(ctx: Arc<RwLock<TypeMap>>, key: String) -> String {
        let data_read = ctx.read().await;
        let rz_meta_lock =
            data_read.get::<RzMeta>().expect("Expected RzMeta in TypeMap.").clone();
        let rz_meta = rz_meta_lock.read().await;
        match rz_meta.get(key.as_str()) {
            Some(x) => x.to_string(),
            None => "".to_string()
        }
    }

    pub async fn set_meta_value(ctx: Arc<RwLock<TypeMap>>, key: String, value: String) {
        let rz_meta_lock = {
            let data_read = ctx.read().await;
            data_read.get::<RzMeta>().expect("Expected RzMeta in TypeMap.").clone()
        };
        let mut rz_meta = rz_meta_lock.write().await;
        rz_meta.insert(key, value);
    }

    pub async fn registry_command(ctx: Arc<RwLock<TypeMap>>, command: String) {
        let rz_commands_lock = {
            let data_read = ctx.read().await;
            data_read.get::<RzCommands>().expect("Expected RzCommands in TypeMap.").clone()
        };
        let mut rz_commands = rz_commands_lock.write().await;
        rz_commands.push(command);
    }

    pub async fn get_commands(ctx: Arc<RwLock<TypeMap>>) -> Vec<String> {
        let rz_commands_lock = {
            let data_read = ctx.read().await;
            data_read.get::<RzCommands>().expect("Expected RzCommands in TypeMap.").clone()
        };
        let rz_commands = rz_commands_lock.read().await;
        let mut commands = Vec::new();
        for command in rz_commands.iter() {
            commands.push(String::from(command));
        }
        return commands;
    }
}