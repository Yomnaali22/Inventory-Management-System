use crate::{
    inventory::{Inventory, Product},
    parse_data::JsonData,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SaleTransaction {
    pub product_sold: String,
    pub quantity_sold: f64,
    pub sale_price: f64,
}

impl SaleTransaction {
    pub fn calculate_total_sales(inventory: &Inventory, product: &Product) -> f64 {
        let mut total_sales = 0.0;
        for sale_transaction in inventory.sales.iter() {
            if sale_transaction.product_sold == product.name {
                total_sales += sale_transaction.sale_price;
            };
        }
        println!("Total sales: {}", total_sales);
        total_sales
    }

    pub fn calculate_profit(product: &Product, inventory: &Inventory) -> f64 {
        let mut profit = 0.0;
        for sale_transaction in inventory.sales.iter() {
            if sale_transaction.product_sold == product.name {
                profit +=
                    sale_transaction.sale_price - product.price * sale_transaction.quantity_sold;
            }
        }
        println!("Total Profit: {}", profit);
        profit
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PurchaseTransaction {
    pub product_purchased: String,
    pub quantity_purchased: f64,
    pub purchase_price: f64,
}

impl PurchaseTransaction {
    pub fn calculate_total_cost(product: &Product, inventory: &Inventory) -> f64 {
        let mut total_cost = 0.0;
        for purchase_transaction in inventory.purchases.iter() {
            if purchase_transaction.product_purchased == product.name {
                total_cost += purchase_transaction.purchase_price;
            }
        }
        println!("Total cost: {}", total_cost);
        total_cost
    }
}
pub trait Transaction {
    fn add_transaction(
        self,
        product: &Product,
        inventory: &mut Inventory,
    ) -> Result<String, String>;
}

impl Transaction for SaleTransaction {
    fn add_transaction(
        self,
        product: &Product,
        inventory: &mut Inventory,
    ) -> Result<String, String> {
        let eligible_for_sale = product.quantity > 0.0;
        let find_sale = inventory
            .sales
            .iter()
            .any(|sale| *sale.product_sold == product.name);
        if eligible_for_sale {
            match find_sale {
                true => {
                    for sale in inventory.sales.iter_mut() {
                        if sale.product_sold == product.name {
                            sale.quantity_sold = self.quantity_sold + sale.quantity_sold;
                            sale.sale_price = self.sale_price + sale.sale_price;
                        }
                    }
                }
                false => {
                    inventory.sales.push(SaleTransaction {
                        product_sold: product.name.clone(),
                        sale_price: self.sale_price,
                        quantity_sold: self.quantity_sold,
                    });
                }
            }
            match JsonData::writes("inventory_data.json", inventory) {
                Ok(_) => println!("Inventory sales data updated successfully."),
                Err(e) => println!("Error updating inventory sales data: {}", e),
            };
            Ok(format!("Product '{}' successfully sold", product.name))
        } else {
            Err(format!("Product '{}' is out of stock", product.name))
        }
    }
}
impl Transaction for PurchaseTransaction {
    fn add_transaction(
        self,
        product: &Product,
        inventory: &mut Inventory,
    ) -> Result<String, String> {
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
        match JsonData::writes("inventory_data.json", inventory) {
            Ok(_) => println!("Inventory Purchases updated successfully."),
            Err(e) => println!("Error updating inventory Purchases data: {}", e),
        };
        Ok(format!("Product '{}' is restocked!", product.name))
    }
}
