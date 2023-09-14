// "use client";
//
// import { ChangeEvent, Dispatch, SetStateAction, useState } from "react";
// import { invoke } from "@tauri-apps/api/tauri";
// import Link from "next/link";
//
export function Credentials({
  onChange,
}: {
  onChange: (id: string, value: string) => void;
}) {
  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    const text = e.target.value;
    const id = e.target.id;
    onChange(id, text);
  };
  return (
    <div>
      <label>
        <div>Username</div>
        <input type="text" id="username" onChange={(e) => handleChange(e)} />
      </label>
      <label>
        <div>Password</div>
        <input
          type="password"
          id="password"
          onChange={(e) => handleChange(e)}
        />
      </label>
    </div>
  );
}

// export default function Login({
//   setToken,
// }: {
//   setToken: Dispatch<SetStateAction<string>>;
// }) {
//   // hardoced for now
//   const [credentials, setCredentials] = useState({
//     ns: process.env.ns,
//     db: process.env.db,
//     sc: process.env.sc,
//     host: process.env.host,
//     port: process.env.port,
//   });
//   const handleCredentialsChange = (fieldId: string, value: string) => {
//     setCredentials({ ...credentials, [fieldId]: value });
//   };
//   const handleSubmit = async (e: React.FormEvent) => {
//     e.preventDefault();
//
//     invoke<string>("login", { credentials })
//       .then((message) => console.log(message))
//       .catch((error) => console.error(error));
//   };
//
//   return (
//     <div>
//       <Link href="/">Back</Link>
//       <h1 className="text-3xl font-bold">Sign In</h1>
//       <p className="text-xl font-bold">Please sign in to continue</p>
//       <form onSubmit={handleSubmit}>
//         <Credentials onChange={handleCredentialsChange} />
//         <div>
//           <button type="submit">Submit</button>
//         </div>
//       </form>
//     </div>
//   );
// }
// import { Alert, Box, Button, Snackbar } from "@mui/material";
// import Grid from "@mui/material/Grid";
// import TextField from "@mui/material/TextField";
// import axios from "axios";
// import Image from "next/image";
// import { useRouter } from "next/navigation";
// import { useState } from "react";
// import BobLogo from "../../../public/logo.svg";
// import style from "./page.module.css";
//
// const Login = () => {
//   const router = useRouter();
//   const [address, setAddress] = useState("");
//   const [port, setPort] = useState("");
//   const [username, setUsername] = useState("");
//   const [password, setPassword] = useState("");
//   const [openSnackbar, setOpenSnackbar] = useState(false);
//   const [snackbarMessage, setSnackbarMessage] = useState("");
//
//   function removeEmpty(obj: any) {
//     for (let key of Object.keys(obj)) {
//       if (obj[key] === "") {
//         delete obj[key];
//       } else if (typeof obj[key] === "object") {
//         obj[key] = removeEmpty(obj[key]);
//         if (Object.keys(obj[key]).length === 0) delete obj[key];
//       }
//     }
//     return Array.isArray(obj) ? obj.filter((val) => val) : obj;
//   }
//
//   const snackbarStyle = {
//     top: "20%",
//     left: "50%",
//     transform: "translate(-50%, -50%)",
//   };
//
//   const handleSubmit = (e: any) => {
//     e.preventDefault();
//     axios
//       .post(
//         "http://localhost:7000/api/login",
//         removeEmpty({
//           hostname: address + ":" + port,
//           credentials: {
//             login: username,
//             password: password,
//           },
//         }),
//         { withCredentials: true },
//       )
//       .then((res) => {
//         router.push("/dashboard");
//       })
//       .catch((err) => {
//         setSnackbarMessage("Wrong data");
//         setOpenSnackbar(true);
//         console.log(err.message);
//       });
//   };
//
//   return (
//     <Box className={style.main}>
//       <Box
//         component="form"
//         onSubmit={handleSubmit}
//         className={style.form}
//         sx={{
//           width: 400,
//           padding: "30px",
//           borderRadius: "8px",
//           boxShadow: "0px 4px 8px rgba(0, 0, 0, 0.1)",
//         }}
//       >
//         <Box sx={{ display: "flex", justifyContent: "center", mb: 4 }}>
//           <Image src={BobLogo} width={300} height={60} alt="BobLogo" />
//         </Box>
//         <Snackbar
//           open={openSnackbar}
//           autoHideDuration={3000}
//           onClose={() => setOpenSnackbar(false)}
//           style={snackbarStyle}
//         >
//           <Alert onClose={() => setOpenSnackbar(false)} severity="error">
//             {snackbarMessage}
//           </Alert>
//         </Snackbar>
//         <Grid container spacing={2}>
//           <Grid item xs={12} sm={8}>
//             <TextField
//               onChange={(e) => setAddress(e.target.value)}
//               required
//               fullWidth
//               label="Адрес"
//               name="address"
//               id="address"
//               autoFocus
//               variant="outlined"
//             />
//           </Grid>
//           <Grid item xs={12} sm={4}>
//             <TextField
//               onChange={(e) => setPort(e.target.value)}
//               required
//               fullWidth
//               label="Порт"
//               name="port"
//               id="port"
//               variant="outlined"
//             />
//           </Grid>
//           <Grid item xs={12}>
//             <TextField
//               onChange={(e) => setUsername(e.target.value)}
//               fullWidth
//               label="Логин"
//               variant="outlined"
//             />
//           </Grid>
//           <Grid item xs={12}>
//             <TextField
//               onChange={(e) => setPassword(e.target.value)}
//               fullWidth
//               label="Пароль"
//               type="password"
//               variant="outlined"
//             />
//           </Grid>
//         </Grid>
//         <Box sx={{ display: "flex", justifyContent: "center", mt: 4 }}>
//           <Button
//             type="submit"
//             variant="contained"
//             color="primary"
//             size="large"
//             sx={{ width: "100%" }}
//           >
//             Авторизоваться
//           </Button>
//         </Box>
//       </Box>
//     </Box>
//   );
// };
//
// export default Login;
