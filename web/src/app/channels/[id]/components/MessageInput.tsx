export default function MessageInput() {
  return (
    <div className="mb-2 flex h-12 w-full flex-grow">
      <input
        className="mx-2 h-full grow rounded-full bg-gray-200 px-4 text-sm text-gray-900 outline-none"
        type="text"
        id="message"
        name="message"
        placeholder="Send a message..."
      />
    </div>
  );
}
