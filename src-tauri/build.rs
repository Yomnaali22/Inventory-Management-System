use dotenv::{dotenv, from_filename};
use std::env;

fn main() {
    // load env variables from .env file
    from_filename(".env").ok();
    // Now you can access variables
    tauri_build::build();
}
