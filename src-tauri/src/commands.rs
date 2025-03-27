use axum::Json;
use inventory_lib::{add_product, get_inventory, Inventory, Product};
use serde_json::Value;
use tauri::command;

#[command]
pub fn inventory() -> Value {
    let Json(inventory) = get_inventory();
    inventory
}

#[command]
pub fn add_product_to_inventory(product: Product) -> Value {
    let Json(inventory) = add_product(&product);
    inventory
}
