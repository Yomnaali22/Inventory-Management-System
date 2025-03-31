use crate::{Inventory, JsonData, Product, handle_env_variables};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct SaleTransaction {
    pub product_sold: String,
    pub quantity_sold: f64,
    pub sale_price: f64, // for one product
    pub total_sales: f64,
    pub total_profit: f64,
}
fn calculate_total_sales(sale_transaction: &SaleTransaction) -> f64 {
    sale_transaction.sale_price * sale_transaction.quantity_sold
}

fn calculate_profit(sale_transaction: &SaleTransaction, product: &Product) -> f64 {
    let mut profit = 0.0;
    profit += sale_transaction.sale_price - product.price * sale_transaction.quantity_sold;
    profit
}

impl SaleTransaction {
    pub fn add_sale_transaction(&self, inventory: &mut Inventory) -> Result<String, String> {
        let product = inventory
            .products
            .iter()
            .find(|product| product.name == self.product_sold);

        if let Some(product) = product {
            let eligible_for_sale = product.quantity > 0.0;
            let find_sale = inventory
                .sales
                .iter()
                .any(|sale| sale.product_sold == product.name);
            println!("eleigible for sale: {}", eligible_for_sale);

            if eligible_for_sale {
                match find_sale {
                    true => {
                        // check if product is in inventory
                        for sale in inventory.sales.iter_mut() {
                            if sale.product_sold == product.name {
                                sale.quantity_sold += self.quantity_sold;
                                sale.sale_price += self.sale_price;
                                // calculate total sales when quantity sold increase
                                sale.total_sales += calculate_total_sales(&self);
                                // calculate profit when quantity sold increase
                                sale.total_profit += calculate_profit(&self, &product);
                            }
                        }
                    }
                    false => {
                        inventory.sales.push(SaleTransaction {
                            product_sold: product.name.clone(),
                            quantity_sold: self.quantity_sold,
                            sale_price: self.sale_price,
                            total_sales: calculate_total_sales(&self),
                            total_profit: calculate_profit(&self, &product),
                        });
                    }
                }

                let path = handle_env_variables("INVENTORY_JSON_PATH");
                match JsonData::writes(&path, inventory) {
                    Ok(_) => println!("Inventory sales data updated successfully."),
                    Err(e) => println!("Error updating inventory sales data: {}", e),
                };
                Ok(format!("Product '{}' successfully sold", product.name))
            } else {
                Err(format!("Product '{}' is out of stock", product.name))
            }
        } else {
            Err(format!(
                "Not eligible to sell product. Product not found in inventory",
            ))
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PurchaseTransaction {
    pub product_purchased: String,
    pub quantity_purchased: f64,
    pub purchase_price: f64,
    pub total_cost: f64,
}

fn calculate_total_cost(purchase_transaction: &PurchaseTransaction) -> f64 {
    purchase_transaction.purchase_price * purchase_transaction.quantity_purchased
}

impl PurchaseTransaction {
    pub fn add_purchase_transaction(&self, inventory: &mut Inventory) -> Result<String, String> {
        let product = inventory
            .products
            .iter()
            .find(|product| product.name == self.product_purchased);
        if let Some(product) = product {
            let find_purchase = inventory
                .purchases
                .iter()
                .any(|purchase| *purchase.product_purchased == product.name);
            match find_purchase {
                true => {
                    for purchase in inventory.purchases.iter_mut() {
                        if purchase.product_purchased == product.name {
                            purchase.quantity_purchased =
                                self.quantity_purchased + purchase.quantity_purchased;
                            purchase.purchase_price = self.purchase_price + purchase.purchase_price;
                            purchase.total_cost += calculate_total_cost(&self);
                        }
                    }
                }
                false => {
                    inventory.purchases.push(PurchaseTransaction {
                        product_purchased: product.name.clone(),
                        purchase_price: self.purchase_price,
                        quantity_purchased: self.quantity_purchased,
                        total_cost: calculate_total_cost(&self),
                    });
                }
            }
            let path = handle_env_variables("INVENTORY_JSON_PATH");
            match JsonData::writes(&path, inventory) {
                Ok(_) => println!("Inventory Purchases updated successfully."),
                Err(e) => println!("Error updating inventory Purchases data: {}", e),
            };
            Ok(format!("Product '{}' is restocked!", product.name))
        } else {
            Err(format!(
                "Not eligible to purchase product. Product not found in inventory",
            ))
        }
    }
}
