"use client";

import store from "./store";
import { Provider as BaseProvider } from "react-redux";

export function Provider({ children }: { children: React.ReactNode }) {
  return <BaseProvider store={store}>{children}</BaseProvider>;
}
