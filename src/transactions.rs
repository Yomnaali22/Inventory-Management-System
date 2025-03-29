use crate::{Inventory, JsonData};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SaleTransaction {
    pub product_sold: String,
    pub quantity_sold: f64,
    pub sale_price: f64,
}

impl SaleTransaction {
    pub fn calculate_total_sales(&self, inventory: &Inventory) -> f64 {
        let mut total_sales = 0.0;
        for sale_transaction in inventory.sales.iter() {
            if sale_transaction.product_sold == self.product_sold {
                total_sales += sale_transaction.sale_price;
            };
        }
        println!("Total sales: {}", total_sales);
        total_sales
    }

    pub fn calculate_profit(&self, inventory: &Inventory) -> Result<f64, String> {
        let mut profit = 0.0;
        let product = inventory
            .products
            .iter()
            .find(|product| product.name == self.product_sold);
        if let Some(product) = product {
            for sale_transaction in inventory.sales.iter() {
                if sale_transaction.product_sold == self.product_sold {
                    profit += sale_transaction.sale_price
                        - product.price * sale_transaction.quantity_sold;
                }
            }
            println!("Total profit: {}", profit);
        } else {
            return Err("Product not found in inventory".to_string());
        }
        Ok(profit)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PurchaseTransaction {
    pub product_purchased: String,
    pub quantity_purchased: f64,
    pub purchase_price: f64,
}

impl PurchaseTransaction {
    pub fn calculate_total_cost(&self, inventory: &Inventory) -> f64 {
        inventory
            .products
            .iter()
            .find(|p| p.name == self.product_purchased)
            .map(|product| {
                inventory
                    .purchases
                    .iter()
                    .filter(|p| p.product_purchased == product.name)
                    .map(|p| p.purchase_price)
                    .sum()
            })
            .unwrap_or(0.0) // Default to 0.0 if product not found
    }
}
pub trait Transaction {
    fn add_transaction(&self, inventory: &mut Inventory, path: &str) -> Result<String, String>;
}

impl Transaction for SaleTransaction {
    fn add_transaction(&self, inventory: &mut Inventory, path: &str) -> Result<String, String> {
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
                        for sale in inventory.sales.iter_mut() {
                            if sale.product_sold == product.name {
                                sale.quantity_sold += self.quantity_sold;
                                sale.sale_price += self.sale_price;
                            }
                        }
                    }
                    false => {
                        inventory.sales.push(SaleTransaction {
                            product_sold: product.name.clone(),
                            quantity_sold: self.quantity_sold,
                            sale_price: self.sale_price,
                        });
                    }
                }
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
impl Transaction for PurchaseTransaction {
    fn add_transaction(&self, inventory: &mut Inventory, path: &str) -> Result<String, String> {
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
                        }
                    }
                }
                false => {
                    inventory.purchases.push(PurchaseTransaction {
                        product_purchased: product.name.clone(),
                        purchase_price: self.purchase_price,
                        quantity_purchased: self.quantity_purchased,
                    });
                }
            }
            match JsonData::writes(path, inventory) {
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
