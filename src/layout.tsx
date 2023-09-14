// import { RefreshContext } from "@/refreshContext";
import Navbar from "@/components/navbar/navbar";
import "./global.css";
import ThemeRegistry from "./themeRegistry";

export const metadata = {
  title: "BobUI",
  description: "User interface for Bob (c) qoollo",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <ThemeRegistry options={{ key: "mui" }}>
          <Navbar />
          {children}
        </ThemeRegistry>
      </body>
    </html>
  );
}
