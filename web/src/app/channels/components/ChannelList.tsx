"use client";

import { useParams } from "next/navigation";
import ChannelItem from "./ChannelItem";
import ChannelListHeader from "./ChannelListHeader";
import { useAppSelector } from "@/hooks";
import { selectChannelList } from "@/state/channels";

export default function ChannelList() {
  const params = useParams();
  const channels = useAppSelector((state) => selectChannelList(state.channels));

  const activeChannelId = parseInt(params.id as string);

  return (
    <div className="flex h-screen w-96 flex-col border-r-2 border-solid py-3 pl-3">
      <ChannelListHeader />
      <div className="flex h-full w-full flex-col overflow-auto pr-3">
        {channels.map((channel) => (
          <ChannelItem
            key={channel.id}
            channel={channel}
            active={channel.id === activeChannelId}
          />
        ))}
      </div>
    </div>
  );
}
