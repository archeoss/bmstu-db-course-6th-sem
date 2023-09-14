// "use client";
//
// import { useEffect } from "react";
// import { invoke } from "@tauri-apps/api/tauri";
// import "@/app/globals.css";
// import SignUp from "@/components/signup";
// import Link from "next/link";
//
// export default function Page() {
//   useEffect(() => {
//     invoke<string>("greet", { name: "Next.js" })
//       .then(console.log)
//       .catch(console.error);
//   }, []);
//
//   return (
//     <main className="flex flex-col justify-between items-center p-24 min-h-screen">
//       <div className="z-10 justify-between items-center w-full max-w-5xl font-mono text-sm lg:flex">
//         <Link href="/">Back</Link>
//       </div>
//       <div>
//         <h1 className="text-3xl font-bold">Sign Up</h1>
//       </div>
//       <div>
//         <SignUp></SignUp>
//       </div>
//     </main>
//   );
// }
"use client";

import { Alert, Box, Button, Snackbar } from "@mui/material";
import Grid from "@mui/material/Grid";
import TextField from "@mui/material/TextField";
import { useRouter } from "next/navigation";
import { ChangeEvent, Dispatch, SetStateAction, useState } from "react";
import style from "./page.module.css";
import { invoke } from "@tauri-apps/api";
import { Role } from "@/types/types.d";

export function RoleSelector({
  onChange,
  initRole,
}: {
  onChange: Dispatch<SetStateAction<Role>>;
  initRole: Role;
}) {
  return (
    <p>
      <select
        defaultValue={initRole}
        size={3}
        onChange={(e) => onChange(Role[e.target.value as keyof typeof Role])}
      >
        <option value={Role.Human} id="human">
          {Role[Role.Human]}
        </option>
        <option value={Role.Computer} id="computer">
          {Role[Role.Computer]}
        </option>
        <option value={Role.Interrogator} id="interrogator">
          {Role[Role.Interrogator]}
        </option>
      </select>
    </p>
  );
}

const Signup = () => {
  const [openSnackbar, setOpenSnackbar] = useState(false);
  const [snackbarMessage, setSnackbarMessage] = useState("");

  function removeEmpty(obj: any) {
    for (let key of Object.keys(obj)) {
      if (obj[key] === "") {
        delete obj[key];
      } else if (typeof obj[key] === "object") {
        obj[key] = removeEmpty(obj[key]);
        if (Object.keys(obj[key]).length === 0) delete obj[key];
      }
    }
    return Array.isArray(obj) ? obj.filter((val) => val) : obj;
  }

  const snackbarStyle = {
    top: "20%",
    left: "50%",
    transform: "translate(-50%, -50%)",
  };

  const [credentials, setCredentials] = useState({
    ns: process.env.ns,
    db: process.env.db,
    sc: process.env.sc,
    host: process.env.host,
    port: process.env.port,
  });
  const [role, setRole] = useState(Role.Human);

  const handleCredentialsChange = (fieldId: string, value: string) => {
    setCredentials({ ...credentials, [fieldId]: value });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    invoke<string>("signup", { credentials, role: role.toString() })
      .then((message) => {
        console.log(credentials, role), localStorage.setItem("token", message);
      })
      .catch((error) => console.error(error));
  };

  return (
    <Box className={style.main}>
      <Box
        component="form"
        onSubmit={handleSubmit}
        className={style.form}
        sx={{
          width: 400,
          padding: "30px",
          borderRadius: "8px",
          boxShadow: "0px 4px 8px rgba(0, 0, 0, 0.1)",
        }}
      >
        {/* <Box sx={{ display: "flex", justifyContent: "center", mb: 4 }}> */}
        {/*   <Image src={BobLogo} width={300} height={60} alt="BobLogo" /> */}
        {/* </Box> */}
        <Snackbar
          open={openSnackbar}
          autoHideDuration={3000}
          onClose={() => setOpenSnackbar(false)}
          style={snackbarStyle}
        >
          <Alert onClose={() => setOpenSnackbar(false)} severity="error">
            {snackbarMessage}
          </Alert>
        </Snackbar>
        <Grid container spacing={2}>
          <Grid item xs={12}>
            <TextField
              id="username"
              onChange={(e) =>
                handleCredentialsChange(e.target.id, e.target.value)
              }
              fullWidth
              label="Логин"
              variant="outlined"
            />
          </Grid>
          <Grid item xs={12}>
            <TextField
              id="password"
              onChange={(e) =>
                handleCredentialsChange(e.target.id, e.target.value)
              }
              fullWidth
              label="Пароль"
              type="password"
              variant="outlined"
            />
          </Grid>
          <Grid item xs={12}>
            <RoleSelector onChange={setRole} initRole={role} />
          </Grid>
        </Grid>
        <Box sx={{ display: "flex", justifyContent: "center", mt: 4 }}>
          <Button
            type="submit"
            variant="contained"
            color="primary"
            size="large"
            sx={{ width: "100%" }}
          >
            Авторизоваться
          </Button>
        </Box>
      </Box>
    </Box>
  );
};

export default Signup;
