"use client";

import { useEffect, useState } from "react";
import { Box } from "@mui/material";
import { invoke } from "@tauri-apps/api";
import { User } from "@/types/types";

const NodeIdPage = () => {
  const [token, setToken] = useState("");
  const [name, setName] = useState("");
  const [role, setRole] = useState("");

  useEffect(() => {
    setToken(localStorage.getItem("token") || "");
  }, [setToken]);
  useEffect(() => {
    console.log(token);
    if (token) {
      const context = {
        ns: process.env.ns,
        db: process.env.db,
        sc: process.env.sc,
        host: process.env.host,
        port: process.env.port,
      };
      invoke<User>("get_info", { context, token })
        .then((res) => {
          setName(res.user);
          setRole(res.role);
        })
        .catch(console.error);
    }
  });
  <Box
    sx={{
      marginTop: "24px",
      marginBottom: "24px",
    }}
  ></Box>;
};
