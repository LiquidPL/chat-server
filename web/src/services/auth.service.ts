import axios, { AxiosError, AxiosResponse } from "axios";

import { redirect } from "next/navigation";

import getConfig from "../config";
import { User } from "@/models";
import { setAccessToken, setUser, unsetUser } from "../state/user";
import store from "@/store";

interface Login {
  username: string;
  password: string;
}

interface LoginResponse {
  user?: User;
  token?: string;
  error?: string;
}

function loadFromLocalStorage(): User | undefined {
  if (typeof window === "undefined") {
    return;
  }

  const localLogin = window.localStorage.getItem("login");

  if (localLogin === null) {
    return;
  }

  const login: LoginResponse = JSON.parse(localLogin);

  if (login.user === undefined || login.token === undefined) {
    return;
  }

  store.dispatch(setAccessToken(login.token));
  store.dispatch(setUser(login.user));

  return login.user;
}

async function login(
  username: string,
  password: string,
): Promise<string | undefined> {
  try {
    const loginResponse: AxiosResponse<LoginResponse> = await axios.post(
      getConfig().apiUrl + "/users/login",
      null,
      {
        params: { username, password },
      },
    );

    if (
      loginResponse.data.token === undefined ||
      loginResponse.data.user === undefined
    ) {
      return "error";
    }

    store.dispatch(setAccessToken(loginResponse.data.token));
    store.dispatch(setUser(loginResponse.data.user));

    window.localStorage.setItem("login", JSON.stringify(loginResponse.data));
  } catch (e) {
    const response = (e as AxiosError).response?.data as LoginResponse;

    return response.error ?? "error";
  }
}

function logout() {
  axios.get(getConfig().apiUrl + "/users/logout").then(() => {
    store.dispatch(unsetUser());
    redirect("/login");
  });
}

export const authService = {
  login,
  logout,
  loadFromLocalStorage,
};
