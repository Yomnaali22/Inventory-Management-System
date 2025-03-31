// use serde_json::{Value, json};
// use tokio::net::TcpListener;
// mod transactions;
// use axum::{Json, Router, http::Method, routing::get};
// use tower_http::cors::{Any, CorsLayer};

// imported from the root lib package (lib file)
use shared::{Inventory, JsonData, PurchaseTransaction, SaleTransaction, process_user_input};

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

    let sale_transaction = SaleTransaction {
        product_sold: "Laptop".to_string(),
        sale_price: 1500.0,
        quantity_sold: 3.0,
        total_sales: 0.0,
        total_profit: 0.0,
    };

    match SaleTransaction::add_sale_transaction(&sale_transaction, &mut inventory) {
        Ok(_) => println!("Sale transaction added successfully."),
        Err(err) => return Err(err.to_string()),
    };
    let purchase_transaction = PurchaseTransaction {
        product_purchased: "Laptop".to_string(),
        purchase_price: 1500.0,
        quantity_purchased: 3.0,
        total_cost: 0.0, // add purchase will calculate for us
    };
    match PurchaseTransaction::add_purchase_transaction(&purchase_transaction, &mut inventory) {
        Ok(_) => println!("Purchase transaction added successfully."),
        Err(err) => return Err(err.to_string()),
    }
    Inventory::generate_report(&inventory);

    Ok(())
}
