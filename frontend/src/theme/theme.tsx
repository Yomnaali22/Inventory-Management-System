import { createTheme } from "@mui/material/styles";
const darkTheme = createTheme({
  palette: {
    mode: "dark",
    primary: {
      main: "#e47553", // Futuristic green accent
    },
    secondary: {
      main: "#ff4081", // Vibrant pink accent
    },
    background: {
      default: "#121212",
      paper: "#1e1e1e",
    },
    text: {
      primary: "#ffffff",
      secondary: "#b0b0b0",
    },
  },
  typography: {
    fontFamily: "Fira Code",
    h1: {
      fontFamily: "Orbitron",
      fontSize: "2.5rem",
      fontWeight: "bold",
    },
    h2: {
      fontSize: "1.5rem",
      fontWeight: "bold",
    },
    h6: {
      fontSize: "0.9rem",
      fontWeight: "620",
    },

    body1: {
      fontSize: "1rem",
    },
    body2: {
      fontSize: "0.875rem",
    },
  },

  components: {
    MuiCssBaseline: {
      styleOverrides: {
        root: {
          transition: "none",
          "&hover": {
            transition: "none",
          },
        },
        body: {
          background:
            "linear-gradient(135deg, #121212 0%, #1e1e1e 50%, #2a2a2a 100%)",
          backgroundImage:
            "radial-gradient(circle at 50% 50%, rgba(228, 117, 83, 0.1) 0%, transparent 50%)",
          minHeight: "100vh",
        },
      },
    },
  },
});

export default darkTheme;
