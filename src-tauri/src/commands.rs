use serde_json::{Value, json};
use shared::{
    Inventory, Product, PurchaseTransaction, SaleTransaction, User, add_product, add_purchase,
    add_sale, authenticate, get_inventory,
};
use std::fs;
use tauri::command;

#[command]
pub fn inventory() -> Value {
    let inventory: Result<Inventory, String> = get_inventory();
    let inventory = match inventory {
        Ok(inventory) => inventory,
        Err(e) => return json!(e),
    };

    json!(inventory)
}

#[command]
pub fn add_product_to_inventory(product: Product) -> Value {
    let inventory = add_product(&product);
    inventory
}

#[command]
pub fn login(credentials: User) -> Result<Value, String> {
    let auth: Result<User, String> = match authenticate(&credentials.name, &credentials.password) {
        Ok(user) => Ok(user),
        Err(e) => return Err(format!("Login Failed: {}", e)),
    };
    Ok(json!(auth))
}

#[command]
pub fn record_sale_transaction(sale_transaction: SaleTransaction) -> Value {
    let sale = add_sale(sale_transaction);
    sale
}

#[command]
pub fn record_purchase_transaction(purchase_transaction: PurchaseTransaction) -> Value {
    let purchase = add_purchase(purchase_transaction);
    purchase
}

#[command]
pub fn export_report(file_name: String) -> Result<String, String> {
    let inventory = get_inventory()?;
    let path = format!("reports/{}", file_name);
    let body = serde_json::to_string_pretty(&inventory).map_err(|e| e.to_string())?;
    fs::write(&path, body).map_err(|e| e.to_string())?;
    Ok(path)
}
