pub mod app;
pub use app::{add_product, add_purchase, add_sale, get_inventory};
pub mod inventory;
pub use inventory::{Inventory, Product};
pub mod parse_data;
pub use parse_data::JsonData;
pub mod transactions;
pub use transactions::{PurchaseTransaction, SaleTransaction, Transaction};
pub mod user_auth;
pub use user_auth::{User, authenticate, process_user_input};
pub mod env;
pub use env::get_env_var;
pub fn handle_env_variables(key: &str) -> String {
    if let Some(env) = get_env_var(key) {
        env
    } else {
        println!("Error getting env");
        "Error getting env".to_string()
    }
}
