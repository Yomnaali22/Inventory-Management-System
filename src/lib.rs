pub mod app;
pub use app::{add_product, get_inventory};
pub mod inventory;
pub use inventory::{Inventory, Product};
pub mod parse_data;
pub mod transactions;
