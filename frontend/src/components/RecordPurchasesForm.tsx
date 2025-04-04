import React from "react";
import { Box, TextField, Button, Typography, Grid } from "@mui/material";
import { PurchaseTransaction } from "../types";

interface RecordPurchaseFormProps {
  onSubmit: (purchase: PurchaseTransaction) => void;
  onCancel: () => void;
}

const RecordPurchaseForm: React.FC<RecordPurchaseFormProps> = ({
  onSubmit,
  onCancel,
}) => {
  const [purchase, setPurchase] = React.useState<PurchaseTransaction>({
    product_purchased: "",
    quantity_purchased: 0,
    purchase_price: 0.0,
    total_cost: 0.0,
  });

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setPurchase(prev => ({
      ...prev,
      [name]:
        name === "quantity_purchased" || name === "purchase_price"
          ? parseFloat(value)
          : value,
      total_cost: 0.0, // Reset or you could calculate it here
    }));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit(purchase);
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
        Record Purchase
      </Typography>
      <Grid container spacing={3}>
        <TextField
          fullWidth
          label="Product Purchased"
          name="product_purchased"
          value={purchase.product_purchased}
          onChange={handleChange}
          required
        />
        <TextField
          fullWidth
          label="Quantity Purchased"
          name="quantity_purchased"
          type="number"
          inputProps={{ step: "0.01" }}
          value={purchase.quantity_purchased}
          onChange={handleChange}
          required
        />
        <TextField
          fullWidth
          label="Purchase Price"
          name="purchase_price"
          type="number"
          inputProps={{ step: "0.01" }}
          value={purchase.purchase_price}
          onChange={handleChange}
          required
        />
      </Grid>
      <Box sx={{ display: "flex", justifyContent: "center", mt: 3, gap: 2 }}>
        <Button variant="outlined" onClick={onCancel}>
          <Typography variant="h6">Cancel</Typography>
        </Button>
        <Button variant="contained" type="submit">
          <Typography variant="h6">Record Purchase</Typography>
        </Button>
      </Box>
    </Box>
  );
};
export default RecordPurchaseForm;
