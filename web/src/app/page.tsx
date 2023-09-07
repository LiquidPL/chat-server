"use client";

import Link from "next/link";

export default function Home() {
  return (
    <div className="flex h-screen w-screen items-center justify-center">
      <div className="flex flex-col items-center">
        <h1 className="mb-4 text-4xl font-semibold text-gray-900">Chat App</h1>
        <Link className="text-xl text-indigo-600" href="/login">
          Log in
        </Link>
      </div>
    </div>
  );
}
