// src/types.ts
export interface Product {
  name: string;
  quantity: number;
  price: number;
  description: string;
}

export interface Sale {
  total_sales: number;
  total_profit: number;
}

export interface Purchase {
  total_cost: number;
}

export interface Inventory {
  products: Product[] | [];
  sales: (saleTransaction & Sale)[] | [];
  purchases: (PurchaseTransaction & Purchase)[] | [];
}

export interface LoginResponse {
  loggedIn: boolean;
  errorMessage: string | null;
}
export interface saleTransaction {
  product_sold: string;
  quantity_sold: number;
  sale_price: number;
}

export interface PurchaseTransaction {
  product_purchased: string;
  quantity_purchased: number;
  purchase_price: number;
}

export type FormType =
  | "addProduct"
  | "recordSale"
  | "recordPurchase"
  | "report"
  | null;
