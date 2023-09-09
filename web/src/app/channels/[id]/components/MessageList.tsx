"use client";

import MessageItem from "./MessageItem";
import { useAppSelector } from "@/hooks";
import { selectMessagesByChannelId } from "@/state/messages";
import { useEffect, useRef } from "react";

export default function MessageList({ id: channel_id }: { id: number }) {
  const messages = useAppSelector((state) =>
    selectMessagesByChannelId(state, channel_id),
  );
  const listRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const messageElements = listRef?.current?.childNodes;

    if (messageElements === undefined || messageElements.length === 0) {
      return;
    }

    const lastMessageElement = messageElements[messageElements.length - 1];

    (lastMessageElement as HTMLElement).scrollIntoView();
  }, [messages]);

  return (
    <div
      className="flex h-full w-full flex-col overflow-auto p-3"
      ref={listRef}
    >
      {messages.map((message) => (
        <MessageItem key={message.id} message={message} />
      ))}
    </div>
  );
}
