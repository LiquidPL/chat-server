import { User } from "@/models";
import pluralize from "pluralize";
import ChannelMember from "./ChannelMember";
import { useAppSelector } from "@/hooks";
import { selectUsersByChannelId } from "@/state/users";
import { useState } from "react";
import { Dialog } from "@headlessui/react";
import axios from "axios";
import getConfig from "@/config";
import { error } from "console";

export default function ChannelMemberList({ id }: { id: number }) {
  const [isOpen, setIsOpen] = useState(false);
  const [username, setUsername] = useState("");
  const [error, setError] = useState("");
  const accessToken = useAppSelector((state) => state.auth.accessToken);

  const members = useAppSelector((state) =>
    selectUsersByChannelId(state, id),
  ) as User[];

  const inviteUser = (username: string) => {
    axios
      .post(
        getConfig().apiUrl + "/channels/" + id + "/invite",
        { username },
        {
          headers: {
            Authorization: "Bearer " + accessToken,
          },
        },
      )
      .then((response) => {
        setIsOpen(false);
      })
      .catch((error) => {
        setError(error.response.data.error);
      });
  };

  return (
    <div className="flex h-screen w-96 flex-col overflow-auto border-l-2 border-solid p-3">
      <p>
        <span className="pl-2 text-xs text-gray-600">
          {pluralize("members", members.length, true)}&nbsp;&#8226;&nbsp;
        </span>
        <a
          className="text-xs text-indigo-600"
          href="#"
          onClick={() => setIsOpen(true)}
        >
          Invite
        </a>
      </p>
      {members.map((member) => (
        <ChannelMember key={member.id} member={member} />
      ))}

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
              Invite a user
            </Dialog.Title>

            <input
              className="mb-2 mt-2 h-10 rounded-md px-1.5 text-sm text-gray-900 shadow-sm outline-none ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-inset focus:ring-indigo-600"
              placeholder="Enter username"
              onChange={(e) => setUsername(e.target.value)}
            />

            {error.length > 0 ? (
              <span className="mb-4 mt-2 text-sm font-medium text-red-600">
                {error}
              </span>
            ) : (
              ""
            )}

            <div className="flex justify-between">
              <button
                className="cursor-pointer rounded-md bg-red-600 p-3 text-sm font-semibold text-white hover:bg-red-500"
                onClick={() => setIsOpen(false)}
              >
                Cancel
              </button>
              <button
                className="cursor-pointer rounded-md bg-indigo-600 p-3 text-sm font-semibold text-white hover:bg-indigo-500"
                onClick={() => inviteUser(username)}
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
