"use client";

export default function ChannelHeader({ name }: { name: string }) {
  return (
    <div className="flex h-16 w-full items-center border-b-2 border-solid px-6 font-semibold">
      <span className="text-2xl">{name}</span>
    </div>
  );
}
