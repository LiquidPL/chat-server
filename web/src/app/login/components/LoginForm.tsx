import { authService } from "@/services/auth.service";
import { useState } from "react";
import { useRouter } from "next/navigation";

export default function LoginForm() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const router = useRouter();

  let errorMessage: React.ReactNode;

  if (error.length > 0) {
    errorMessage = (
      <span className="my-2 text-sm font-medium text-red-600">{error}</span>
    );
  } else {
    errorMessage = [];
  }

  return (
    <div className="flex w-full flex-col">
      <h1 className="mb-16 text-center text-4xl font-bold">Log in</h1>

      <label htmlFor="username" className="text-sm font-medium">
        Username
      </label>
      <input
        className="mb-6 mt-2 h-10 rounded-md px-1.5 text-sm text-gray-900 shadow-sm outline-none ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-inset focus:ring-indigo-600"
        onChange={(e) => setUsername(e.target.value)}
        type="text"
        id="username"
        name="username"
        required
      />

      <label htmlFor="password" className="text-sm font-medium">
        Password
      </label>
      <input
        className="mb-6 mt-2 h-10 rounded-md px-1.5 text-sm text-gray-900 shadow-sm outline-none ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-inset focus:ring-indigo-600"
        onChange={(e) => setPassword(e.target.value)}
        type="password"
        id="password"
        name="password"
        required
      />

      {errorMessage}

      <input
        className="mt-2 h-10 cursor-pointer rounded-md bg-indigo-600 text-sm font-semibold text-white hover:bg-indigo-500"
        onClick={() => {
          authService.login(username, password).then((error) => {
            if (error !== undefined) {
              setError(error);
            } else {
              router.push("/channels");
            }
          });
        }}
        type="button"
        value="Sign in"
      />
    </div>
  );
}
