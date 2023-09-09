"use client";

import { useAppSelector } from "@/hooks";
import { useEffect } from "react";
import { redirect } from "next/navigation";
import { authService } from "@/services/auth.service";

export default function AuthGuard({ children }: { children: React.ReactNode }) {
  useEffect(() => {
    const user = authService.loadFromLocalStorage();

    if (user === undefined) {
      redirect("/login");
    }
  });

  return children;
}
