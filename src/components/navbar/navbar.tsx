"use client";

import ExitToAppIcon from "@mui/icons-material/ExitToApp";
import {
  AppBar,
  Box,
  FormControl,
  FormControlLabel,
  IconButton,
  InputLabel,
  MenuItem,
  Select,
  Switch,
  Tab,
  Tabs,
  Toolbar,
} from "@mui/material";
import axios from "axios";
import Image from "next/image";
import Link from "next/link";
import { usePathname, useRouter } from "next/navigation";
import { useEffect, useState } from "react";

export let refreshTime: number;
export let switcher: boolean = false;

const Navbar = () => {
  const router = useRouter();

  const [value, setValue] = useState(0);

  useEffect(() => {
    const path = window.location.pathname;
    if (path === "/dashboard") {
      setValue(0);
    } else if (path === "/nodelist") {
      setValue(1);
    } else if (path === "/vdisklist") {
      setValue(2);
    }
  }, []);

  const [refresh, setRefresh] = useState(1);
  const [switchButton, setSwitchButton] = useState(false);

  refreshTime = refresh;
  switcher = switchButton;

  const pathname = usePathname();
  if (pathname === "/login" || pathname === "/") {
    return <div></div>;
  }

  const handleLogout = () => {
    router.push("/");
  };

  const Spacer = () => <Box sx={{ flexGrow: 1 }} />;

  return (
    <AppBar
      position="static"
      style={{ background: "#000000", padding: "16px 22px" }}
      sx={{ width: "100%" }}
    >
      <Toolbar>
        <Box
          sx={{
            display: "flex",
            flexDirection: "row",
            alignItems: "center",
            gap: "45px",
            width: "60%",
          }}
        >
          <Tabs
            indicatorColor="primary"
            textColor="secondary"
            value={value}
            onChange={(e, val) => setValue(val)}
          >
            <Tab
              label="Account"
              component={Link}
              href="account"
              style={{ textTransform: "none", fontSize: "16px" }}
            />
            {/* <Tab */}
            {/*   label="Ноды" */}
            {/*   component={Link} */}
            {/*   href="nodelist" */}
            {/*   style={{ textTransform: "none", fontSize: "16px" }} */}
            {/* /> */}
            {/* <Tab */}
            {/*   label="Виртуальные диски" */}
            {/*   component={Link} */}
            {/*   href="vdisklist" */}
            {/*   style={{ textTransform: "none", fontSize: "16px" }} */}
            {/* /> */}
          </Tabs>
        </Box>
        <Box
          sx={{
            display: "flex",
            flexDirection: "row",
            alignItems: "center",
            justifyContent: "flex-end",
            gap: "25px",
            width: "40%",
          }}
        >
          <FormControlLabel
            value="stop"
            control={<Switch onChange={(e, c) => setSwitchButton(c)} />}
            label="STOP"
            labelPlacement="start"
            sx={{
              "&.MuiFormControlLabel-labelPlacementStart": {
                color: "#FF6936",
              },
            }}
          />
          <span>Обновлять каждые</span>
          <FormControl
            variant="standard"
            sx={{
              width: "85px",
            }}
          >
            <InputLabel id="polling-label-id" style={{ marginTop: "-20px" }}>
              мин
            </InputLabel>
            <Select
              style={{ marginTop: "-5px" }}
              labelId="polling-select-label-id"
              id="pollint-select-id"
              label="min"
              value={refresh}
              onChange={(e) => setRefresh(e.target.value as number)}
            >
              {/* NOTE: время в минутах */}
              <MenuItem value={1}>1</MenuItem>
              <MenuItem value={5}>5</MenuItem>
              <MenuItem value={15}>15</MenuItem>
              <MenuItem value={30}>30</MenuItem>
            </Select>
          </FormControl>
          <Box
            sx={{
              marginTop: "-10px",
              cursor: "pointer",
              marginLeft: "+70px",
            }}
          >
            <IconButton onClick={handleLogout} color="inherit">
              <ExitToAppIcon fontSize="large" color="primary" />
            </IconButton>
          </Box>
        </Box>
      </Toolbar>
    </AppBar>
  );
};

export default Navbar;
