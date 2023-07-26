// "use client";

import {
  ChangeEvent,
  Dispatch,
  SetStateAction,
  useEffect,
  useState,
} from "react";
import { invoke } from "@tauri-apps/api/tauri";

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

export default function Login({
  setToken,
}: {
  setToken: Dispatch<SetStateAction<string>>;
}) {
  // hardoced for now
  const [credentials, setCredentials] = useState({
    ns: "TuringApp",
    db: "TuringDB",
    sc: "TuringScope",
    host: "localhost",
    port: 8080,
  });
  const handleCredentialsChange = (fieldId: string, value: string) => {
    setCredentials({ ...credentials, [fieldId]: value });
  };
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    invoke<string>("login", { credentials })
      .then((message) => console.log(message))
      .catch((error) => console.error(error));
  };

  return (
    <form onSubmit={handleSubmit}>
      <Credentials onChange={handleCredentialsChange} />
      <div>
        <button type="submit">Submit</button>
      </div>
    </form>
  );
}
