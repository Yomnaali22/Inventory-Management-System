// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use dotenv::{dotenv, from_filename};

fn main() {
    from_filename(".env").ok();
    let my_var = std::env::var("INVENTORY_JSON_PATH").expect("INVENTORY_JSON_PATH must be set");
    println!("INVENTORY_JSON_PATH: {}", my_var);
    app_lib::run();
}
