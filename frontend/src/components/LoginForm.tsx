import React from "react";
import { Box, TextField, Button, Typography, Grid } from "@mui/material";

interface LoginFormProps {
  onSubmit: (credentials: { username: string; password: string }) => void;
  onCancel?: () => void;
}

const LoginForm: React.FC<LoginFormProps> = ({ onSubmit, onCancel }) => {
  const [credentials, setCredentials] = React.useState({
    username: "",
    password: "",
  });

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setCredentials(prev => ({
      ...prev,
      [name]: value,
    }));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit(credentials);
  };

  return (
    <Box component="form" onSubmit={handleSubmit} sx={{ mt: 2 }}>
      <Typography
        variant="h6"
        gutterBottom
        sx={{
          p: 2,
        }}
      >
        Login
      </Typography>
      <Grid container spacing={3}>
        <TextField
          fullWidth
          label="Username"
          name="username"
          value={credentials.username}
          onChange={handleChange}
          required
          autoFocus
        />

        <TextField
          fullWidth
          label="Password"
          name="password"
          type="password"
          value={credentials.password}
          onChange={handleChange}
          required
        />
      </Grid>
      <Box sx={{ display: "flex", justifyContent: "center", mt: 3, gap: 2 }}>
        {onCancel && (
          <Button variant="outlined" onClick={onCancel}>
            Cancel
          </Button>
        )}
        <Button variant="contained" type="submit">
          Login
        </Button>
      </Box>
    </Box>
  );
};

export default LoginForm;
