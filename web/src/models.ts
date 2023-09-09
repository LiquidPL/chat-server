export interface User {
  id: number;
  username: string;
}

export interface Auth {
  token: string;
}

export interface UserAuthenticated {
  user: User,
  channels: Channel[],
}

export interface Event<T> {
  event_type: "Auth"|"UserAuthenticated";
  data: T;
}

export interface Channel {
  id: number;
  name: string;
  owner_id: number;
  created_at: string;
  updated_at: string;
}
