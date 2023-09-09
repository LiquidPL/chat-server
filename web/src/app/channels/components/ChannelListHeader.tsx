"use client";

import { useState } from "react";
import { Dialog } from "@headlessui/react";
import axios from "axios";
import getConfig from "@/config";
import { Channel, NewChannel } from "@/models";
import { useAppSelector } from "@/hooks";
import { selectAuthenticatedUser } from "@/state/auth";
import { useRouter } from "next/navigation";

export default function ChannelListHeader() {
  const [isOpen, setIsOpen] = useState(false);
  const [newChannelName, setNewChannelName] = useState("");

  const router = useRouter();

  const user = useAppSelector((state) => selectAuthenticatedUser(state));
  const accessToken = useAppSelector((state) => state.auth.accessToken);

  const createChannel = (channelName: string) => {
    axios
      .post<Channel>(
        getConfig().apiUrl + "/channels",
        { name: channelName, owner_id: user?.id } as NewChannel,
        {
          headers: {
            Authorization: "Bearer " + accessToken,
          },
        },
      )
      .then((response) => {
        setIsOpen(false);
        router.push("/channels/" + response.data.id);
      });
  };

  return (
    <div className="flex h-16 items-center justify-between p-3">
      <span className="text-2xl font-bold">Channels</span>
      <button
        className="aspect-square h-full rounded-full bg-indigo-600 text-2xl text-white"
        onClick={() => setIsOpen(true)}
      >
        +
      </button>

      <Dialog open={isOpen} onClose={() => setIsOpen(false)}>
        <div className="fixed inset-0 bg-black/30" aria-hidden={true} />

        <div className="fixed inset-0 flex w-screen items-center justify-center p-4">
          <Dialog.Panel
            as="div"
            className="overlow-hidden my-8 flex w-full max-w-md flex-col rounded-md bg-white p-6 text-left align-middle shadow-xl"
          >
            <Dialog.Title
              as="h3"
              className="mb-4 text-lg font-medium text-gray-900"
            >
              Create a new channel
            </Dialog.Title>

            <input
              className="mb-6 mt-2 h-10 rounded-md px-1.5 text-sm text-gray-900 shadow-sm outline-none ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-inset focus:ring-indigo-600"
              placeholder="Enter channel name"
              onChange={(e) => setNewChannelName(e.target.value)}
            />

            <div className="flex justify-between">
              <button
                className="cursor-pointer rounded-md bg-red-600 p-3 text-sm font-semibold text-white hover:bg-red-500"
                onClick={() => setIsOpen(false)}
              >
                Cancel
              </button>
              <button
                className="cursor-pointer rounded-md bg-indigo-600 p-3 text-sm font-semibold text-white hover:bg-indigo-500"
                onClick={() => createChannel(newChannelName)}
              >
                Confirm
              </button>
            </div>
          </Dialog.Panel>
        </div>
      </Dialog>
    </div>
  );
}
