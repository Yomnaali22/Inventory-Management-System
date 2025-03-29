use lib::{
    add_product, add_purchase, add_sale, authenticate, get_inventory, Inventory, Product,
    PurchaseTransaction, SaleTransaction, User,
};
use serde_json::{json, Value};
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
