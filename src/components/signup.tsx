// "use client";

import {
  ChangeEvent,
  Dispatch,
  SetStateAction,
  useEffect,
  useState,
} from "react";
import { invoke } from "@tauri-apps/api/tauri";
import SignIn, { Credentials } from "./login";
import { ReactPropTypes } from "react";

export function RoleSelector({
  onChange,
}: {
  onChange: Dispatch<SetStateAction<string>>;
}) {
  const [role, setRole] = useState("Human");
  const onRoleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setRole(e.target.value);
  };
  return (
    <label>
      <div>Role</div>
      <input
        type="radio"
        name="role"
        value="Human"
        id="human"
        checked={role === "Human"}
        onChange={(e) => onChange(e.target.value)}
      />
      <label>Human</label>
      <input
        type="radio"
        name="role"
        value="Computer"
        id="computer"
        checked={role === "Computer"}
        onChange={(e) => onChange(e.target.value)}
      />
      <label>Computer</label>
      <input
        type="radio"
        name="role"
        value="Interrogator"
        id="interrogator"
        checked={role === "Interrogator"}
        onChange={(e) => onChange(e.target.value)}
      />
      <label>Interrogator</label>
    </label>
  );
}

export default function SignUp() {
  const [credentials, setCredentials] = useState({
    ns: "TuringApp",
    db: "TuringDB",
    sc: "TuringScope",
    host: "localhost",
    port: 8080,
  });
  const [role, setRole] = useState("");
  const handleCredentialsChange = (fieldId: string, value: string) => {
    setCredentials({ ...credentials, [fieldId]: value });
  };
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    console.log(role, credentials);
    // invoke<string>("signup", { credentials })
    //   .then((message) => console.log(message))
    //   .catch((error) => console.error(error));
  };
  return (
    <form onSubmit={handleSubmit}>
      <Credentials onChange={handleCredentialsChange} />
      <RoleSelector onChange={setRole} />
      <div>
        <button type="submit">Submit</button>
      </div>
    </form>
  );
}
