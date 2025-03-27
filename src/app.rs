use crate::parse_data::JsonData;
use crate::{Product, inventory::Inventory};
use axum::Json;
use serde_json::{Value, json};

pub fn get_inventory() -> Json<Value> {
    let inventory: Inventory = match JsonData::process("./../inventory_data.json") {
        Ok(data) => data,
        Err(err) => {
            println!("Error processing inventory data: {:?}", err);
            return Json(json!("error"));
        }
    };
    // Generate report and convert to JSON
    Json(json!(inventory))
}

pub fn add_product(product: &Product) -> Json<Value> {
    let mut inventory: Inventory = match JsonData::process("./../inventory_data.json") {
        Ok(data) => data,
        Err(err) => {
            println!("Error processing inventory data: {:?}", err);
            return Json(json!("Error processing inventory data: "));
        }
    };
    let inventory = match Inventory::modify_products(product, "add", &mut inventory) {
        Ok(data) => String::from(data),
        Err(err) => {
            println!("Error adding product: {:?}", err);
            return Json(json!("error adding product"));
        }
    };
    // Generate report and convert to JSON
    Json(json!(inventory))
}
