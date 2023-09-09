"use client";

import { Message, User } from "@/models";
import { useAppSelector } from "@/hooks";
import UserAvatar from "./UserAvatar";
import { selectAuthenticatedUser } from "@/state/auth";
import { selectUserById } from "@/state/users";

export default function MessageItem({ message }: { message: Message }) {
  const user = useAppSelector((state) => selectAuthenticatedUser(state));
  const sender = useAppSelector((state) =>
    selectUserById(state, message.sender_id),
  ) ?? { id: -1, username: "unknown user" };

  const isPostedByUser = user?.id === message.sender_id;

  const backgroundColor = isPostedByUser ? "bg-blue-600" : "bg-gray-100";
  const margin = isPostedByUser ? "ml-auto" : "mr-auto";
  const textColor = isPostedByUser ? "text-white" : "text-gray-900";

  return (
    <div className="mb-2 flex flex-col">
      {createUsername(sender, isPostedByUser)}
      <div className="flex w-full">
        {isPostedByUser ? <></> : createAvatar(sender, isPostedByUser)}
        <div
          className={`h-9 max-w-prose rounded-full px-3 py-1.5 ${textColor} ${backgroundColor} ${margin}`}
        >
          {message.content}
        </div>
        {isPostedByUser ? createAvatar(sender, isPostedByUser) : <></>}
      </div>
    </div>
  );
}

function createAvatar(sender: User, isPostedByUser: boolean) {
  const margin = isPostedByUser ? "ml-2" : "mr-2";

  return (
    <div className={`aspect-square ${margin} translate-y-2`}>
      <UserAvatar user={sender} />
    </div>
  );
}

function createUsername(sender: User, isPostedByUser: boolean) {
  const marginLeft = isPostedByUser ? "ml-auto" : "ml-14";
  const marginRight = isPostedByUser ? "mr-14" : "mr-auto";

  return (
    <span className={`${marginLeft} ${marginRight} text-sm text-gray-400`}>
      {sender.username}
    </span>
  );
}
