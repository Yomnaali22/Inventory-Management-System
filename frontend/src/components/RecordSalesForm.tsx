import React from "react";
import { Box, TextField, Button, Typography, Grid } from "@mui/material";
import { saleTransaction } from "../types";

interface RecordSaleFormProps {
  onSubmit: (sale: saleTransaction) => void;
  onCancel: () => void;
}

const RecordSaleForm: React.FC<RecordSaleFormProps> = ({
  onSubmit,
  onCancel,
}) => {
  const [sale, setSale] = React.useState<saleTransaction>({
    product_sold: "",
    quantity_sold: 0.0,
    sale_price: 0.0,
    total_sales: 0.0,
    total_profit: 0.0,
  });

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const t = e.target;
    setSale({
      product_sold: t.name === "Product sold" ? t.value : sale.product_sold,
      quantity_sold:
        t.name === "Quantity Sold" ? parseFloat(t.value) : sale.quantity_sold,
      sale_price:
        t.name === "Sale Price" ? parseFloat(t.value) : sale.sale_price,
      total_sales: 0.0,
      total_profit: 0.0,
    });
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit(sale);
  };

  return (
    <Box component="form" onSubmit={handleSubmit}>
      <Typography
        variant="h6"
        gutterBottom
        sx={{
          p: 2,
        }}
      >
        Record Sale
      </Typography>
      <Grid container spacing={3}>
        <TextField
          fullWidth
          label="Product Sold"
          name="Product sold"
          value={sale.product_sold}
          onChange={handleChange}
          required
        />
        <TextField
          fullWidth
          label="Quantity Sold"
          name="Quantity Sold"
          type="number"
          inputProps={{ step: "0.01" }}
          value={sale.quantity_sold}
          onChange={handleChange}
          required
        />

        <TextField
          fullWidth
          label="Sale Price"
          name="Sale Price"
          type="number"
          inputProps={{ step: "0.01" }}
          value={sale.sale_price}
          onChange={handleChange}
          required
        />
      </Grid>
      <Box sx={{ display: "flex", justifyContent: "center", mt: 3, gap: 2 }}>
        <Button variant="outlined" onClick={onCancel}>
          <Typography variant="h6">Cancel</Typography>
        </Button>
        <Button variant="contained" type="submit">
          <Typography variant="h6">Record Sale</Typography>
        </Button>
      </Box>
    </Box>
  );
};
export default RecordSaleForm;
