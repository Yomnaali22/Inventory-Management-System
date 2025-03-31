import React, { useState } from "react";
import {
  Box,
  CssBaseline,
  Container,
  Typography,
  Button,
  Paper,
} from "@mui/material";
import { ThemeProvider } from "@mui/material/styles";
import {
  Product,
  FormType,
  LoginResponse,
  PurchaseTransaction,
  saleTransaction,
} from "./types";
import darkTheme from "./theme/theme";
import {
  AddProductForm,
  RecordSaleForm,
  RecordPurchaseForm,
  InventoryReport,
  LoginForm,
} from "./components";
import "./App.css";
import {
  add_product,
  login,
  record_purchase_transaction,
  record_sale_transaction,
} from "./api";
const App: React.FC = () => {
  const [activeForm, setActiveForm] = useState<FormType>(null);
  const [loginSuccess, setLoginSuccess] = useState<LoginResponse>({
    loggedIn: false,
    errorMessage: null,
  });

  const handleAddProduct = (product: Product) => {
    setActiveForm(null);
    add_product(product);
  };
  const handleRecordSale = (sale: saleTransaction) => {
    record_sale_transaction({
      ...sale,
      total_sales: 0.0,
      total_profit: 0.0,
    });
    setActiveForm(null);
  };

  const handleRecordPurchase = async (purchase: PurchaseTransaction) => {
    record_purchase_transaction({
      ...purchase,
      total_cost: 0.0,
    });
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
        return <InventoryReport onClose={() => setActiveForm(null)} />;
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
          {loginSuccess.loggedIn ? (
            renderForm()
          ) : (
            <LoginForm
              onSubmit={credentials => {
                login(credentials, setLoginSuccess);
              }}
            />
          )}
          {!loginSuccess.loggedIn ? (
            <Typography variant="body1" color={"error"} sx={{ mt: 3 }}>
              {loginSuccess.errorMessage}
            </Typography>
          ) : null}
        </Paper>
      </Container>
    </ThemeProvider>
  );
};

export default App;
