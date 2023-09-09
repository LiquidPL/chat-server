"use client";

import { Channel, Message } from "@/models";
import Link from "next/link";

export default function ChannelItem({
  channel,
  message,
  active = false,
}: {
  channel: Channel;
  message?: Message;
  active?: boolean;
}) {
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
          <span className="text-sm text-gray-600">{message?.content}</span>
          <span className="text-sm text-gray-600">yesterday</span>
        </div>
      </div>
    </Link>
  );
}
