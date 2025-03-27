import React from "react";
import { Box, TextField, Button, Typography, Grid } from "@mui/material";
import { Product } from "../types";

interface AddProductFormProps {
  onSubmit: (product: Product) => void;
  onCancel: () => void;
}

const AddProductForm: React.FC<AddProductFormProps> = ({
  onSubmit,
  onCancel,
}) => {
  const [product, setProduct] = React.useState<Omit<Product, "id">>({
    name: "",
    quantity: 0,
    price: 0,
    description: "",
  });

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setProduct(prev => ({
      ...prev,
      [name]:
        name === "quantity" || name === "price"
          ? parseFloat(value) || 0
          : value,
    }));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit(product);
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
        Add New Product
      </Typography>
      <Grid container spacing={3}>
        <TextField
          fullWidth
          label="Product Name"
          name="name"
          value={product.name}
          onChange={handleChange}
          required
        />

        <TextField
          fullWidth
          label="Quantity"
          name="quantity"
          type="number"
          inputProps={{ step: "0.01" }}
          value={product.quantity}
          onChange={handleChange}
          required
        />
        <TextField
          fullWidth
          label="Price"
          name="price"
          type="number"
          inputProps={{ step: "0.01" }}
          value={product.price}
          onChange={handleChange}
          required
        />
        <TextField
          fullWidth
          label="Description"
          name="description"
          multiline
          rows={3}
          value={product.description}
          onChange={handleChange}
        />
      </Grid>
      <Box sx={{ display: "flex", justifyContent: "center", mt: 3, gap: 2 }}>
        <Button variant="outlined" onClick={onCancel}>
          <Typography variant="h6">Cancel</Typography>
        </Button>
        <Button variant="contained" type="submit">
          <Typography variant="h6">Add Product</Typography>
        </Button>
      </Box>
    </Box>
  );
};

export default AddProductForm;
