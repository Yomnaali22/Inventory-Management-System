use serde_json::{Value, json};
use tokio::net::TcpListener;
mod transactions;
use axum::{Json, Router, http::Method, routing::get};
use tower_http::cors::{Any, CorsLayer};

// imported from the root lib package (lib file)
use lib::JsonData;
use lib::process_user_input;
use lib::{Inventory, Product};
use lib::{PurchaseTransaction, SaleTransaction, Transaction};
// ✅ Move the async server logic into a separate function

// ✅ Same function structure you provided, but with CORS added
// async fn run_server() {
//     // ✅ Enable CORS
//     let cors = CorsLayer::new()
//         .allow_origin(Any) // Allow all origins (for development)
//         .allow_methods([Method::GET, Method::POST]); // Allow necessary methods
//     let app = Router::new()
//         .route("/inventory", get(get_inventory)) // ✅ Fixed async function reference
//         .layer(cors); // ✅ Apply CORS middleware

//     let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
//     println!("🚀 Server running at http://127.0.0.1:3000");

//     axum::serve(listener, app).await.unwrap();
// }
fn main() -> Result<(), String> {
    // let rt = tokio::runtime::Runtime::new().unwrap();
    // rt.block_on(run_server());
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

    sale_transaction.calculate_total_sales(&inventory); // calculate_total_sales has access the sale transaction through self
    SaleTransaction::calculate_profit(&sale_transaction, &inventory)?;

    Inventory::generate_report(&inventory);

    Ok(())
}
