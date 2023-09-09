import getConfig from "@/config";
import { useAppSelector } from "@/hooks";
import { NewMessage } from "@/models";
import axios from "axios";
import { useParams } from "next/navigation";
import { useState } from "react";

export default function MessageInput() {
  const [message, setMessage] = useState("");
  const accessToken = useAppSelector((state) => state.user.accessToken);
  const params = useParams();

  const activeChannelId = parseInt(params.id as string);

  return (
    <div className="mb-2 flex h-12 w-full flex-grow">
      <input
        className="mx-2 h-full grow rounded-full bg-gray-200 px-4 text-sm text-gray-900 outline-none"
        type="text"
        id="message"
        name="message"
        placeholder="Send a message..."
        onChange={(event) => setMessage(event.target.value)}
        onKeyDown={(event) => {
          if (event.key === "Enter") {
            sendMessage(activeChannelId, message, accessToken);
            (event.target as HTMLInputElement).value = "";
          }
        }}
      />
    </div>
  );
}

function sendMessage(channelId: number, message: string, accessToken?: string) {
  if (accessToken === undefined) {
    return;
  }

  axios.post(
    getConfig().apiUrl + "/channels/" + channelId + "/messages",
    { content: message } as NewMessage,
    {
      headers: {
        "Content-Type": "application/json",
        Authorization: "Bearer " + accessToken,
      },
    },
  );
}
