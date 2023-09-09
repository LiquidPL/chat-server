"use client";

import MessageList from "./components/MessageList";
import ChannelHeader from "./components/Header";
import ChannelMemberList from "./components/ChannelMemberList";
import MessageInput from "./components/MessageInput";
import { useEffect } from "react";
import axios from "axios";
import getConfig from "@/config";
import { useAppDispatch, useAppSelector } from "@/hooks";
import { Message } from "@/models";
import { addMessages } from "@/state/messages";

export default function Channel({ params }: { params: { id: number } }) {
  const accessToken = useAppSelector((state) => state.user.accessToken);
  const dispatch = useAppDispatch();

  useEffect(() => {
    if (accessToken === undefined) {
      return;
    }

    axios
      .get(getConfig().apiUrl + "/channels/" + params.id + "/messages", {
        headers: {
          Authorization: "Bearer " + accessToken,
        },
      })
      .then((response) => {
        const messages: Message[] = response.data;
        dispatch(addMessages(messages));
      });
  });

  return (
    <div className="flex h-full w-full">
      <div className="flex h-full w-full flex-col">
        <ChannelHeader name="asdf" />
        <MessageList id={params.id} />
        <MessageInput />
      </div>
    </div>
  );
}
