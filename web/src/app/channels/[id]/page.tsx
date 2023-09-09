"use client";

import MessageList from "./components/MessageList";
import ChannelHeader from "./components/Header";
import ChannelMemberList from "./components/ChannelMemberList";
import MessageInput from "./components/MessageInput";
import { useEffect } from "react";
import { useRouter } from "next/navigation";
import { useSubscribe } from "use-pubsub-js";
import axios, { AxiosError } from "axios";

import getConfig from "@/config";
import { Message } from "@/models";
import { useAppDispatch, useAppSelector } from "@/hooks";
import { addMessages } from "@/state/messages";

export const CHANNEL_DELETED = Symbol("CHANNEL_DELETED");

export default function Channel({ params }: { params: { id: number } }) {
  const accessToken = useAppSelector((state) => state.user.accessToken);
  const dispatch = useAppDispatch();
  const router = useRouter();

  useSubscribe({token: CHANNEL_DELETED, handler: (_, message) => {
    if (message === undefined) {
      return;
    }

    const channelId = parseInt(message);

    if (channelId == params.id) {
      router.push("/channels");
    }
  }});

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
      })
      .catch((error: AxiosError) => {
        if (error.response?.status === 404) {
          router.push("/channels");
        }
      });
  });

  return (
    <div className="flex h-full w-full">
      <div className="flex h-full w-full flex-col">
        <ChannelHeader name="asdf" />
        <MessageList id={params.id} />
        <MessageInput />
      </div>
      <ChannelMemberList />
    </div>
  );
}
