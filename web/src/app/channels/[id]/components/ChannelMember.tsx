import { User } from "@/models";
import UserAvatar from "./UserAvatar";

export default function ChannelMember({ member }: { member: User }) {
  return (
    <div className="flex h-12 cursor-pointer items-center rounded-md px-2 py-1 hover:bg-gray-100">
      <UserAvatar user={member} />
      <span className="text-sm font-medium text-gray-900">
        {member.username}
      </span>
    </div>
  );
}
