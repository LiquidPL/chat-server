export interface User {
  id: number;
  username: string;
}

export interface Channel {
  id: number;
  name: string;
  owner_id: number;
  created_at: string;
  updated_at: string;
}

export interface ChannelListItem {
  channel: Channel;
  mostRecentMessage: Message;
}

export interface Message {
  id: number;
  channel_id: number;
  sender_id: number;
  content: string;
  created_at: string;
}

export interface NewMessage {
  content: string;
}

export interface Auth {
  token: string;
}

export interface InitialChannel {
  channel: Channel;
  message: Message | null;
}

export interface UserAuthenticated {
  user: User;
  channels: InitialChannel[];
}

export type MessageCreated = Message;

export interface Event<T> {
  event_type: "Auth" | "UserAuthenticated" | "MessageCreated";
  data: T;
}
