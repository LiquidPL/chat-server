"use client";

import { Channel, Message } from "@/models";
import { formatDistanceToNow, parseISO } from "date-fns";
import { utcToZonedTime } from "date-fns-tz";
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

  const timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;
  const localDate = message
    ? utcToZonedTime(parseISO(message?.created_at + "+00:00"), timezone)
    : null;

  return (
    <Link href={`/channels/${channel.id}`}>
      <div
        className={`mb-2 flex h-20 cursor-pointer flex-col justify-between rounded-md p-4 hover:bg-gray-100 ${backgroundColor}`}
      >
        <div>
          <span className="text-gray-900">{channel.name}</span>
        </div>
        <div className="align-center flex justify-between overflow-hidden">
          <span className="text-sm text-gray-600">
            {message ? message?.content : "No messages yet"}
          </span>
          <span className="text-sm text-gray-600">
            {localDate ? formatDistanceToNow(localDate) + " ago" : ""}
          </span>
        </div>
      </div>
    </Link>
  );
}
