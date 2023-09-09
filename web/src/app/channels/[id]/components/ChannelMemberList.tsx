import { User } from "@/models";
import pluralize from "pluralize";
import ChannelMember from "./ChannelMember";
import { useAppSelector } from "@/hooks";
import { selectUsersByChannelId } from "@/state/users";

export default function ChannelMemberList({ id }: { id: number }) {
  const members = useAppSelector((state) =>
    selectUsersByChannelId(state, id),
  ) as User[];

  return (
    <div className="flex h-screen w-96 flex-col overflow-auto border-l-2 border-solid p-3">
      <p>
        <span className="pl-2 text-xs text-gray-600">
          {pluralize("members", members.length, true)}&nbsp;&#8226;&nbsp;
        </span>
        <a className="text-xs text-indigo-600" href="#">
          Invite
        </a>
      </p>
      {members.map((member) => (
        <ChannelMember key={member.id} member={member} />
      ))}
    </div>
  );
}
