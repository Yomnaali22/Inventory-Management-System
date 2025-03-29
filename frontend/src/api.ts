import { invoke } from "@tauri-apps/api/core";
import {
  Inventory,
  LoginResponse,
  Product,
  saleTransaction,
  PurchaseTransaction,
} from "./types";

const fetch_inventory = async (
  setInventory: React.Dispatch<React.SetStateAction<Inventory>>
) => {
  try {
    const inventory: Inventory = await invoke("inventory");
    setInventory(inventory);
    return inventory;
  } catch (err) {
    console.error("error fetching inventory: ", err);
    return err;
  }
};

const add_product = async (product: Product) => {
  try {
    const inventory: Inventory = await invoke("add_product_to_inventory", {
      product: product,
    });
    return inventory;
  } catch (err) {
    console.error("error adding product to inventory: ", err);
    return err;
  }
};
const login = async (
  credentials: { name: string; password: string },
  setLoginSuccess: React.Dispatch<React.SetStateAction<LoginResponse>>
) => {
  try {
    await invoke("login", { credentials: credentials });
    setLoginSuccess({
      loggedIn: true,
      errorMessage: null,
    });
  } catch (err) {
    setLoginSuccess({ loggedIn: false, errorMessage: `${err}` });
    console.error("login error: ", err);
  }
};

const record_sale_transaction = async (sale: saleTransaction) => {
  console.log("sale: ", sale);
  try {
    const sale_transaction = await invoke("record_sale_transaction", {
      saleTransaction: sale,
    });
    return sale_transaction;
  } catch (err) {
    console.error("error recording sale transaction: ", err);
    return err;
  }
};

const record_purchase_transaction = async (purchase: PurchaseTransaction) => {
  try {
    const purchase_transaction: PurchaseTransaction = await invoke(
      "record_purchase_transaction",
      {
        purchaseTransaction: purchase,
      }
    );
    return purchase_transaction;
  } catch (err) {
    console.error("error recording purchase transaction: ", err);
    return err;
  }
};
export {
  fetch_inventory,
  add_product,
  login,
  record_sale_transaction,
  record_purchase_transaction,
};
