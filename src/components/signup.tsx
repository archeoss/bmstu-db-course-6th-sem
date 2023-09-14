// "use client";

import { Dispatch, SetStateAction, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Credentials } from "./login";
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

export default function SignUp() {
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
      .then((message) => localStorage.setItem("token", message))
      .catch((error) => console.error(error));
  };
  return (
    <form onSubmit={handleSubmit}>
      <Credentials onChange={handleCredentialsChange} />
      <RoleSelector onChange={setRole} initRole={role} />
      <div>
        <button type="submit">Submit</button>
      </div>
    </form>
  );
}
