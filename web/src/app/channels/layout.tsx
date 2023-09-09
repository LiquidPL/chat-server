"use client";

import { ReactNode, useEffect } from "react";

import ChannelList from "./components/ChannelList";
import AuthGuard from "../components/AuthGuard";
import { webSocketService } from "@/services/websocket.service";

export default function ChannelsLayout({ children }: { children: ReactNode }) {
  useEffect(() => {
    webSocketService.startWebSocket();
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
