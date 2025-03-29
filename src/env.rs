use dotenv::dotenv;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref ENV_VARS: HashMap<String, String> = {
        dotenv::from_filename(".env_dev").ok(); // Load your custom env file
        std::env::vars().collect()
    };
}

pub fn get_env_var(key: &str) -> Option<String> {
    ENV_VARS.get(key).cloned()
}
