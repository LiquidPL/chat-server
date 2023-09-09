"use client";

export default function ChannelListHeader() {
  return (
    <div className="flex h-16 items-center justify-between p-3">
      <span className="text-2xl font-bold">Channels</span>
      <button className="aspect-square h-full rounded-full bg-indigo-600 text-2xl text-white">
        +
      </button>
    </div>
  );
}
