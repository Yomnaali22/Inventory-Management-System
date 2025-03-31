// src/types.ts
export interface Product {
  name: string;
  quantity: number;
  price: number;
  description: string;
}

export interface Inventory {
  products: Product[] | [];
  sales: saleTransaction[] | [];
  purchases: PurchaseTransaction[] | [];
}

export interface LoginResponse {
  loggedIn: boolean;
  errorMessage: string | null;
}
export interface saleTransaction {
  product_sold: string;
  quantity_sold: number;
  sale_price: number; // for one product
  total_sales: number;
  total_profit: number;
}

export interface PurchaseTransaction {
  product_purchased: string;
  quantity_purchased: number;
  purchase_price: number;
  total_cost: number;
}

export type FormType =
  | "addProduct"
  | "recordSale"
  | "recordPurchase"
  | "report"
  | null;
