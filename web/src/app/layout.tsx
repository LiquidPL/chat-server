"use client";

import "./globals.css";
import { Inter } from "next/font/google";

import { Provider } from "@/provider";
import { authService } from "@/services/auth.service";
import { useEffect } from "react";

const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  useEffect(() => {
    authService.loadFromLocalStorage();
  });

  return (
    <html lang="en">
      <body className={inter.className}>
        <Provider>{children}</Provider>
      </body>
    </html>
  );
}
