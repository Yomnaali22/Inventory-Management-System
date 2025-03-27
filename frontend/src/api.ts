import { invoke } from "@tauri-apps/api/core";
import { Inventory, Product } from "./types";
// Sale, Purchase, FormType
const fetch_inventory = async (
  setInventory: React.Dispatch<React.SetStateAction<Inventory>>
) => {
  try {
    const inventory: Inventory = await invoke("inventory");
    console.log("inventory data: ", inventory);
    setInventory(inventory);
    return inventory;
  } catch {
    console.log("error fetching inventory data");
  }
};

const add_product = async (product: Product) => {
  try {
    const inventory: Inventory = await invoke("add_product_to_inventory", {
      product: product,
    });
    console.log("product added to inventory:  ", inventory);
    return inventory;
  } catch {
    console.log("error fetching inventory data");
  }
};

export { fetch_inventory, add_product };
