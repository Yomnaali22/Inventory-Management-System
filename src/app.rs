use crate::Product;
use crate::handle_env_variables;
use crate::{Inventory, JsonData, PurchaseTransaction, SaleTransaction};
use serde_json::{Value, json};

pub fn get_inventory() -> Result<Inventory, String> {
    let path = handle_env_variables("INVENTORY_JSON_PATH");
    let inventory: Inventory = match JsonData::process(&path) {
        Ok(data) => data,
        Err(err) => {
            println!("Error processing inventory data: {:?}", err);
            return Err(format!("Error processing inventory data: {:?}", err));
        }
    };
    // Generate report and convert to JSON
    Ok(inventory)
}

pub fn add_product(product: &Product) -> Value {
    let path = handle_env_variables("INVENTORY_JSON_PATH");
    let mut inventory: Inventory = match JsonData::process(&path) {
        Ok(data) => data,
        Err(err) => {
            println!("Error processing inventory data: {:?}", err);
            return json!("Error processing inventory data: ");
        }
    };
    let inventory = match Inventory::modify_products(product, "add", &mut inventory, &path) {
        Ok(data) => String::from(data),
        Err(err) => {
            println!("Error adding product: {:?}", err);
            return json!("error adding product");
        }
    };
    // Generate report and convert to JSON
    json!(inventory)
}

pub fn add_sale(sale_transaction: SaleTransaction) -> Value {
    let path = handle_env_variables("INVENTORY_JSON_PATH");
    let mut inventory: Inventory = match JsonData::process(&path) {
        Ok(inventory) => inventory,
        Err(e) => return json!(format!("Error processing inventory data: {}", e)),
    };

    return match SaleTransaction::add_sale_transaction(&sale_transaction, &mut inventory) {
        Ok(transaction) => json!(transaction),
        Err(e) => return json!(format!("Error processing inventory data: {}", e)),
    };
}

pub fn add_purchase(purchase_transaction: PurchaseTransaction) -> Value {
    let path = handle_env_variables("INVENTORY_JSON_PATH");
    let mut inventory: Inventory = match JsonData::process(&path) {
        Ok(inventory) => inventory,
        Err(e) => return json!(format!("Error processing inventory data: {}", e)),
    };
    return match PurchaseTransaction::add_purchase_transaction(
        &purchase_transaction,
        &mut inventory,
    ) {
        Ok(transaction) => json!(transaction),
        Err(e) => return json!(e),
    };
}
