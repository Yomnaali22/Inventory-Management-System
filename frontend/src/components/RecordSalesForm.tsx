import React from "react";
import { Box, TextField, Button, Typography, Grid } from "@mui/material";
import { Sale } from "../types";

interface RecordSaleFormProps {
  onSubmit: (sale: Sale) => void;
  onCancel: () => void;
}

const RecordSaleForm: React.FC<RecordSaleFormProps> = ({
  onSubmit,
  onCancel,
}) => {
  const [sale, setSale] = React.useState<Sale>({
    product_sold: "",
    quantity_sold: 0,
    sale_price: 0,
  });

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setSale(prev => ({
      ...prev,
      [name]:
        name === "quantity" || name === "sold_price"
          ? parseFloat(value) || 0
          : value,
    }));
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
          name="product_sold"
          value={sale.product_sold}
          onChange={handleChange}
          required
        />
        <TextField
          fullWidth
          label="Quantity Sold"
          name="quantity"
          type="number"
          inputProps={{ step: "0.01" }}
          value={sale.quantity_sold}
          onChange={handleChange}
          required
        />

        <TextField
          fullWidth
          label="Sale Price"
          name="sold_price"
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
