import { createTheme } from "@mui/material/styles";

const defaultTheme = createTheme({
  palette: {
    background: {
      paper: "#1a1c21",
    },
    primary: {
      main: "#ff6836",
    },
    secondary: {
      main: "#efefef",
    },
    mode: "dark",
  },
  components: {
    MuiFormLabel: {
      styleOverrides: {
        asterisk: { color: "red" },
      },
    },
  },
});

export default defaultTheme;
