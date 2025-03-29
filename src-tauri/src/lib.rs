#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod commands;
use commands::{
    add_product_to_inventory, inventory, login, record_purchase_transaction,
    record_sale_transaction,
};

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            inventory,
            add_product_to_inventory,
            login,
            record_sale_transaction,
            record_purchase_transaction
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
