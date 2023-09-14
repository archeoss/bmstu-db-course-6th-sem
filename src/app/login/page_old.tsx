"use client";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Login from "@/components/login";

import "@/app/globals.css";
export default function Page() {
  // useEffect(() => {
  //   invoke<string>("greet", { name: "Next.js" })
  //     .then(console.log)
  //     .catch(console.error);
  // }, []);
  const [token, setToken] = useState("");

  return (
    <html lang="en">
      <main className="flex flex-col justify-between items-center p-24 min-h-screen">
        <div>
          <Login setToken={setToken}></Login>
        </div>
      </main>
    </html>
  );
}
