"use client";

import { useAppSelector } from "@/hooks";
import { Channel } from "@/models";
import { selectLastMessageByChannelId } from "@/state/messages";
import Link from "next/link";

export default function ChannelItem({
  channel,
  active = false,
}: {
  channel: Channel;
  active?: boolean;
}) {
  const latestMessage = useAppSelector(state => selectLastMessageByChannelId(state.messages, channel.id));

  const backgroundColor = active ? "bg-gray-100" : "";

  return (
    <Link href={`/channels/${channel.id}`}>
      <div
        className={`mb-2 flex h-20 cursor-pointer flex-col justify-between rounded-md p-4 hover:bg-gray-100 ${backgroundColor}`}
      >
        <div>
          <span className="text-gray-900">{channel.name}</span>
        </div>
        <div className="align-center flex justify-between">
          <span className="text-sm text-gray-600">{latestMessage?.content}</span>
          <span className="text-sm text-gray-600">yesterday</span>
        </div>
      </div>
    </Link>
  );
}
