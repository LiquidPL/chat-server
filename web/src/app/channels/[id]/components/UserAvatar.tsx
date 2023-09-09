import { User } from "@/models";

export default function UserAvatar({ user }: { user: User }) {
  return (
    <div className="mr-2 flex aspect-square h-full items-center justify-center rounded-full bg-indigo-600 leading-none">
      <span className="text-xl font-semibold text-white">
        {user.username[0]}
      </span>
    </div>
  );
}
