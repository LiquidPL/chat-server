"use client";

import { Message, User } from "@/models";
import { useAppSelector } from "@/hooks";
import UserAvatar from "./UserAvatar";
import { selectUser } from "@/state/user";

export default function MessageItem({ message }: { message: Message }) {
  const user = useAppSelector((state) => selectUser(state));

  const isPostedByUser = user?.id === message.sender_id;

  const backgroundColor = isPostedByUser ? "bg-blue-600" : "bg-gray-100";
  const margin = isPostedByUser ? "ml-auto" : "mr-auto";
  const textColor = isPostedByUser ? "text-white" : "text-gray-900";

  const author = {
    id: 727,
    username: "asdf",
  } as User;

  return (
    <div className="mb-2 flex flex-col">
      {createUsername(author, isPostedByUser)}
      <div className="flex w-full">
        {isPostedByUser ? <></> : createAvatar(author, isPostedByUser)}
        <div
          className={`h-9 max-w-prose rounded-full px-3 py-1.5 ${textColor} ${backgroundColor} ${margin}`}
        >
          {message.content}
        </div>
        {isPostedByUser ? createAvatar(author, isPostedByUser) : <></>}
      </div>
    </div>
  );
}

function createAvatar(author: User, isPostedByUser: boolean) {
  const margin = isPostedByUser ? "ml-2" : "mr-2";

  return (
    <div className={`aspect-square ${margin} translate-y-2`}>
      <UserAvatar user={author} />
    </div>
  );
}

function createUsername(author: User, isPostedByUser: boolean) {
  const marginLeft = isPostedByUser ? "ml-auto" : "ml-14";
  const marginRight = isPostedByUser ? "mr-14" : "mr-auto";

  return (
    <span className={`${marginLeft} ${marginRight} text-sm text-gray-400`}>
      {author.username}
    </span>
  );
}
