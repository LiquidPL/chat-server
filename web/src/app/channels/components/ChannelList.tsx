"use client";

import ChannelItem from "./ChannelItem";
import ChannelListHeader from "./ChannelListHeader";
import { useAppSelector } from "@/hooks";
import { selectChannelList } from "@/state/channels";

export default function ChannelList() {
  const channels = useAppSelector((state) => selectChannelList(state));

  const activeChannelId = 2;

  return (
    <div className="flex h-screen w-96 flex-col border-r-2 border-solid px-3">
      <ChannelListHeader />
      {channels.map((channel) => (
        <ChannelItem
          key={channel.id}
          channel={channel}
          active={channel.id === activeChannelId}
        />
      ))}
    </div>
  );
}
