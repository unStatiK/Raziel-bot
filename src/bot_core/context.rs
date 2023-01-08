use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;

pub static GLOBAL_CONTEXT: Lazy<Mutex<RzContext>> = Lazy::new(|| {
    Mutex::new(RzContext::init())
});

pub struct RzContext {
    rz_meta: HashMap<String, String>,
}

impl RzContext {
    pub fn init() -> RzContext {
        RzContext {
            rz_meta: HashMap::new()
        }
    }

    pub fn get_meta_value(&mut self, key: String) -> String {
        match self.rz_meta.get(key.as_str()) {
            Some(x) => x.to_string(),
            None => "".to_string()
        }
    }

    pub fn set_meta_value(&mut self, key: String, value: String) {
        self.rz_meta.insert(key, value);
    }
}