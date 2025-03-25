mod user_auth;
use user_auth::process_user_input;
mod inventory;
mod transactions;
use inventory::{Inventory, Product};
mod parse_data;
use parse_data::JsonData;
use transactions::{PurchaseTransaction, SaleTransaction, Transaction};

fn main() -> Result<(), String> {
    // add  authentication
    process_user_input()?;

    let mut inventory: Inventory = match JsonData::process("inventory_data.json") {
        Ok(data) => data,
        Err(err) => return Err(err.to_string()),
    };
    let product: Product = Product {
        name: String::from("Apple"),
        description: String::from("Apple"),
        quantity: 10.0,
        price: 2.0,
    };
    let purchase_transaction: PurchaseTransaction = PurchaseTransaction {
        product_purchased: String::from("Apple"),
        purchase_price: 10.0,
        quantity_purchased: 10.0,
    };
    let sale_transaction = SaleTransaction {
        product_sold: String::from("Apple"),
        sale_price: 10.0,
        quantity_sold: 5.0,
    };

    sale_transaction.add_transaction(&product, &mut inventory)?;

    purchase_transaction.add_transaction(&product, &mut inventory)?;

    SaleTransaction::calculate_total_sales(&inventory, &product);
    SaleTransaction::calculate_profit(&product, &inventory);
    PurchaseTransaction::calculate_total_cost(&product, &inventory);

    Inventory::generate_report("inventory");
    Inventory::generate_report("sales");
    Inventory::generate_report("purchases");

    Ok(())
}

/*
The system should have the following features:

TODO: Error Handling:
The system should have robust error handling capabilities,
including handling of invalid inputs, out-of-stock items, and other possible errors.

TODO: Security:
The system should have basic security measures in place,
such as authentication for store managers to prevent unauthorized access to the inventory, sales, and purchase data.

TODO: User Interface:
The system should have a clear and intuitive text-based user interface that
allows store managers to easily navigate and perform tasks.

To complete this project, you will need to use Rust's basic data types, functions, and control flow structures, as well as concepts such as ownership, borrowing, structs, enums, traits, and error handling. You may also use third-party libraries and crates to enhance the functionality of your system.

technical doc:

*/
