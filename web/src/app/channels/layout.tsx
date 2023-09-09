"use client";

import { ReactNode, useEffect } from "react";
import { useAppSelector } from "@/hooks";
import getConfig from "@/config";
import { UserAuth } from "@/models";

import ChannelList from "./components/ChannelList";
import AuthGuard from "../components/AuthGuard";

export default function ChannelsLayout({ children }: { children: ReactNode }) {
  const accessToken = useAppSelector((state) => state.user.accessToken);

  useEffect(() => {
    if (accessToken !== undefined) {
      let socketUrl = getConfig().apiUrl.replace("http", "ws") + "/websocket";
      let socket = new WebSocket(socketUrl);

      let tokenPayload = {
        event_type: "Auth",
        data: { token: accessToken } as UserAuth,
      };

      console.log(tokenPayload);

      socket.addEventListener("open", function () {
        socket.send(JSON.stringify(tokenPayload));
      });
    }
  });

  return (
    <AuthGuard>
      <div className="flex h-screen w-screen">
        <div>
          <ChannelList />
        </div>
        <div className="w-full">{children}</div>
      </div>
    </AuthGuard>
  );
}
