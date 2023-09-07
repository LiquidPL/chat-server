"use client";

import { useAppSelector } from "@/hooks";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

import LoginForm from "./components/LoginForm";

export default function Login() {
  const router = useRouter();
  const user = useAppSelector((state) => state.user.user);

  useEffect(() => {
    if (user !== undefined) {
      router.push("/");
    }
  });

  return (
    <div className="flex h-screen w-screen flex-initial items-center justify-center">
      <div className="mx-4 h-1/2 w-full md:mx-0 md:max-w-md">
        <LoginForm />
      </div>
    </div>
  );
}
