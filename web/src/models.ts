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
  members: User[];
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

export interface NewChannel {
  name: string;
  owner_id: number;
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

export type ChannelCreated = Channel;

export interface ChannelDeleted {
  id: number;
  name: string;
}

export type MessageCreated = Message;

export interface MessageDeleted {
  id: number;
}

export interface UserJoined {
  channel: Channel;
  user: User;
}

export interface Event<T> {
  event_type:
    | "Auth"
    | "UserAuthenticated"
    | "MessageCreated"
    | "MessageDeleted"
    | "ChannelCreated"
    | "ChannelDeleted"
    | "UserJoined";
  data: T;
}
