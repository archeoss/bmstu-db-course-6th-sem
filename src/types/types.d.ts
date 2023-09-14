enum Role {
  Human = "Human",
  Interrogator = "Interrogator",
  Computer = "Computer",
}

interface User {
  id: string;
  role: string;
  user: string;
  password: string;
}

export { Role, User };
