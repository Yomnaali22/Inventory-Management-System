import React from "react";
import {
  Box,
  Button,
  Typography,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
} from "@mui/material";
import { Inventory } from "../types";

interface InventoryReportProps {
  inventory: Inventory;
  onClose: () => void;
}

const InventoryReport: React.FC<InventoryReportProps> = ({
  inventory,
  onClose,
}) => {
  return (
    <Box sx={{ mt: 2 }} width={"100%"}>
      <Typography
        variant="h6"
        gutterBottom
        sx={{
          p: 2,
        }}
      >
        Inventory Report
      </Typography>

      <Typography variant="subtitle1" sx={{ mt: 3, mb: 1 }}>
        Products
      </Typography>
      <TableContainer component={Paper} sx={{ mb: 3, width: "100%" }}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>Name</TableCell>
              <TableCell align="right">Quantity</TableCell>
              <TableCell align="right">Price</TableCell>
              <TableCell>Description</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {inventory.products.map((product, index) => (
              <TableRow key={index}>
                <TableCell>{product.name}</TableCell>
                <TableCell align="right">{product.quantity}</TableCell>
                <TableCell align="right">{product.price}</TableCell>
                <TableCell>{product.description}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>

      <Typography variant="subtitle1" sx={{ mt: 3, mb: 1 }}>
        Sales
      </Typography>
      {/**
       * 
       *      "product_sold": "Apple",
      "quantity_sold": 50.0,
      "sale_price": 100.0
       */}
      <TableContainer component={Paper} sx={{ mb: 3 }}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>Product Sold</TableCell>
              <TableCell align="right">Quantity Sold</TableCell>
              <TableCell align="right">Sale Price</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {inventory.sales.map((sale, index) => (
              <TableRow key={index}>
                <TableCell>{sale.product_sold}</TableCell>
                <TableCell align="right">{sale.quantity_sold}</TableCell>
                <TableCell align="right">{sale.sale_price}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>

      <Typography variant="subtitle1" sx={{ mt: 3, mb: 1 }}>
        Purchases
      </Typography>
      <TableContainer component={Paper} sx={{ mb: 3 }}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>Product Purchased</TableCell>
              <TableCell align="right">Quantity</TableCell>
              <TableCell align="right">Purchase Price</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {inventory.purchases.map((purchase, index) => (
              <TableRow key={index}>
                <TableCell>{purchase.product_purchased}</TableCell>
                <TableCell align="right">
                  {purchase.quantity_purchased}
                </TableCell>
                <TableCell align="right">{purchase.purchase_price}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>

      <Box sx={{ display: "flex", justifyContent: "center" }}>
        <Button variant="contained" onClick={onClose}>
          <Typography variant="h6">Close</Typography>
        </Button>
      </Box>
    </Box>
  );
};
export default InventoryReport;
