"use client";

import { useAppSelector } from "@/hooks";
import { selectChannelCount } from "@/state/channels";
import { useEffect } from "react";

export default function Channels() {
  const channelCount = useAppSelector((state) => selectChannelCount(state));

  return (
    <div className="flex h-full w-full items-center justify-center">
      <span className="font-gray-100 text-lg font-medium">
        {renderContent(channelCount)}
      </span>
    </div>
  );
}

function renderContent(channelCount: number) {
  if (channelCount === 0) {
    return "No channels available. Join one to start chatting";
  } else if (channelCount > 0) {
    return "Select a channel";
  } else {
    return "You shouldn't be here.";
  }
}
