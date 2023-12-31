import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { RootState } from "../store";
import { User } from "@/models";

export interface UserState {
  user?: User;
  accessToken?: string;
}

const initialState: UserState = {};

export const userSlice = createSlice({
  name: "auth",
  initialState,
  reducers: {
    setAccessToken: (state, action: PayloadAction<string>) => {
      state.accessToken = action.payload;
    },
    setUser: (state, action: PayloadAction<User>) => {
      state.user = action.payload;
    },
    unsetUser: (state) => {
      state.user = undefined;
      state.accessToken = undefined;
    },
  },
});

export const { setAccessToken, setUser, unsetUser } = userSlice.actions;

export const selectAuthenticatedUser = (state: RootState) => state.auth.user;

export default userSlice.reducer;
