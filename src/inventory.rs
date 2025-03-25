use crate::parse_data::JsonData;
use crate::transactions::{PurchaseTransaction, SaleTransaction};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::io::Result;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Product {
    pub name: String,
    pub description: String,
    pub quantity: f64, // unsigned integer
    pub price: f64,    // high precision floating point number
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Inventory {
    pub products: Vec<Product>,
    pub sales: Vec<SaleTransaction>,
    pub purchases: Vec<PurchaseTransaction>,
}

// to prevent duplications or change to a non existent product

impl Inventory {
    pub fn generate_report(report_type: &str) {
        let inventory: Inventory = match JsonData::process("inventory_data.json") {
            Ok(inventory) => inventory,
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        };

        match report_type {
            "inventory" => {
                println!("inventory Report");
                println!("----------------");
                if inventory.products.len() == 0 {
                    println!("No products in inventory");
                } else {
                    for product in inventory.products.iter() {
                        println!(
                            "Name: {} | Description: {} | Price: ${:.2} | Quantity: {}",
                            product.name, product.description, product.price, product.quantity
                        );
                    }
                }
            }
            "sales" => {
                println!("Sales Report");
                println!("----------------");
                if inventory.sales.len() == 0 {
                    println!("No sales in inventory");
                } else {
                    for sale in inventory.sales.iter() {
                        println!(
                            "Sale price: {} | Price sold: ${:.2}",
                            sale.sale_price, sale.quantity_sold
                        );
                    }
                }
            }
            "purchases" => {
                println!("Purchase Report");
                println!("----------------");
                if inventory.purchases.len() == 0 {
                    println!("No purchases in inventory");
                } else {
                    for purchase in inventory.purchases.iter() {
                        println!(
                            "Quantity purchased: {} | Purchase price: ${:.2}",
                            purchase.quantity_purchased, purchase.purchase_price
                        );
                    }
                }
            }

            _ => println!("Invalid report type"),
        }
    }
    pub fn modify_products(product: &Product, operation_type: &str) -> Result<String> {
        // deserialize the existing inventory_data file
        let mut inventory: Inventory = JsonData::process("inventory_data.json")?;
        match operation_type {
            "add" => {
                let product_exits = inventory
                    .products
                    .iter()
                    .any(|p: &Product| p.name == product.name);
                if product_exits {
                    return Ok("Product already exists".to_string());
                }
                inventory.products.push(product.clone());
                // serialize the data after changes made
                JsonData::writes("inventory_data.json", &inventory)?;
                return Ok("Product added successfully".to_string());
            }
            "edit" => {
                let product_exits = inventory
                    .products
                    .iter()
                    .any(|p: &Product| *p.name == *product.name);
                if product_exits {
                    for p in inventory.products.iter_mut() {
                        if p.name == product.name {
                            *p = product.clone();
                            JsonData::writes("inventory_data.json", &inventory)?;
                            return Ok("Product modified successfully".to_string());
                        }
                    }
                }
                return Ok("Failed to edit product".to_string());
            }
            "delete" => {
                let product_exits = inventory
                    .products
                    .iter()
                    .any(|p: &Product| p.name == product.name);
                if product_exits {
                    inventory
                        .products
                        .retain(|p: &Product| *p.name != *product.name);
                    println!("{:?}", inventory);
                    JsonData::writes("inventory_data.json", &inventory)?;
                    return Ok("Product deleted successfully".to_string());
                }
                return Ok("Failed to delete product".to_string());
            }
            _ => {
                return Ok("Failed to add modify product".to_string());
            }
        }
    }
}
