import React, { useState, useEffect } from "react";
import {
  Box,
  CssBaseline,
  Container,
  Typography,
  Button,
  Paper,
} from "@mui/material";
import { ThemeProvider } from "@mui/material/styles";
import { Inventory, Product, Sale, Purchase, FormType } from "./types";
import darkTheme from "./theme/theme";
import {
  AddProductForm,
  RecordSaleForm,
  RecordPurchaseForm,
  InventoryReport,
  LoginForm,
} from "./components";
import "./App.css";
import { fetch_inventory, add_product } from "./api";
const App: React.FC = () => {
  const [activeForm, setActiveForm] = useState<FormType>(null);
  const [loginSuccess, setLoginSuccess] = useState(false);
  const [inventory, setInventory] = useState<Inventory>({
    products: [],
    sales: [],
    purchases: [],
  });

  useEffect(() => {
    fetch_inventory(setInventory);
  }, []);

  const handleAddProduct = (product: Product) => {
    setInventory(prev => ({
      ...prev,
      products: [...prev.products, product],
    }));
    setActiveForm(null);
    add_product(product);
  };

  const handleRecordSale = (sale: Sale) => {
    setInventory(prev => ({
      ...prev,
      sales: [...prev.sales, sale],
    }));
    setActiveForm(null);
  };

  const handleRecordPurchase = (purchase: Purchase) => {
    setInventory(prev => ({
      ...prev,
      purchases: [...prev.purchases, purchase],
    }));
    setActiveForm(null);
  };

  const renderForm = () => {
    switch (activeForm) {
      case "addProduct":
        return (
          <AddProductForm
            onSubmit={handleAddProduct}
            onCancel={() => setActiveForm(null)}
          />
        );
      case "recordSale":
        return (
          <RecordSaleForm
            onSubmit={handleRecordSale}
            onCancel={() => setActiveForm(null)}
          />
        );
      case "recordPurchase":
        return (
          <RecordPurchaseForm
            onSubmit={handleRecordPurchase}
            onCancel={() => setActiveForm(null)}
          />
        );
      case "report":
        return (
          <InventoryReport
            inventory={inventory}
            onClose={() => setActiveForm(null)}
          />
        );
      default:
        return (
          <Box sx={{ mt: 3, textAlign: "center" }}>
            <Typography
              variant="h1"
              gutterBottom
              sx={{
                fontSize: "2rem",
                fontWeight: "bold",
                mb: 4,
              }}
            >
              Inventory management system in rust 🦀
            </Typography>
            <Typography
              variant="h2"
              gutterBottom
              sx={{ mb: 4, fontSize: "1.2rem", fontWeight: "regular" }}
            >
              Choose an option:
            </Typography>
            <Box
              sx={{
                display: "flex",
                flexDirection: "column",
                gap: 3,
                maxWidth: 300,
                margin: "0 auto",
              }}
            >
              <Button
                variant="contained"
                onClick={() => setActiveForm("addProduct")}
                sx={{ py: 1.5 }}
              >
                <Typography variant="h6">Add Products</Typography>
              </Button>
              <Button
                variant="contained"
                onClick={() => setActiveForm("recordSale")}
                sx={{ py: 1.5 }}
              >
                <Typography variant="h6">Record Sales</Typography>
              </Button>
              <Button
                variant="contained"
                onClick={() => setActiveForm("recordPurchase")}
                sx={{ py: 1.5 }}
              >
                <Typography variant="h6">Record Purchases</Typography>
              </Button>
              <Button
                variant="outlined"
                onClick={() => setActiveForm("report")}
                sx={{ py: 1.5 }}
              >
                <Typography variant="h6">Generate Report</Typography>
              </Button>
            </Box>
          </Box>
        );
    }
  };

  return (
    <ThemeProvider theme={darkTheme}>
      <CssBaseline />
      <Container
        maxWidth="xl"
        sx={{
          py: 4,
          display: "flex",
          flexDirection: "column",
          justifyItems: "center",
          alignItems: "center",
          height: "70vh",
        }}
      >
        <Paper
          elevation={3}
          sx={{
            p: 5,
            borderRadius: 2,
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            justifyContent: "center",
            width: "65vw",
            height: "100%",
            overflow: "auto",
          }}
        >
          {!loginSuccess ? (
            <LoginForm
              onSubmit={() => {
                setLoginSuccess(true);
              }}
            />
          ) : (
            renderForm()
          )}
        </Paper>
      </Container>
    </ThemeProvider>
  );
};

export default App;
