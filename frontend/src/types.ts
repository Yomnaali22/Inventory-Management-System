// src/types.ts
export interface Product {
  name: string;
  quantity: number;
  price: number;
  description: string;
}

export interface Sale {
  product_sold: string;
  quantity_sold: number;
  sale_price: number;
}

export interface Purchase {
  product_purchased: string;
  quantity_purchased: number;
  purchase_price: number;
}

export interface Inventory {
  products: Product[] | [];
  sales: Sale[] | [];
  purchases: Purchase[] | [];
}

export type FormType =
  | "addProduct"
  | "recordSale"
  | "recordPurchase"
  | "report"
  | null;
