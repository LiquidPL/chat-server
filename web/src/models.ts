export interface User {
  id: number;
  username: string;
}

export interface UserAuth {
  token: string;
}

export interface Channel {
  id: number;
  name: string;
  owner_id: number;
  created_at: string;
  updated_at: string;
}
