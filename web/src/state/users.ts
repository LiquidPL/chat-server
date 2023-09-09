import { User } from "@/models";
import { RootState } from "@/store";
import {
  createEntityAdapter,
  createSelector,
  createSlice,
} from "@reduxjs/toolkit";
import { selectChannelById } from "./channels";

const usersAdapter = createEntityAdapter<User>();

const initialState = usersAdapter.getInitialState();

const usersSlice = createSlice({
  name: "users",
  initialState,
  reducers: {
    addUser: usersAdapter.upsertOne,
    addUsers: usersAdapter.upsertMany,
  },
});

export const { addUser, addUsers } = usersSlice.actions;

const { selectEntities, selectById } = usersAdapter.getSelectors(
  (state: RootState) => state.users,
);

const selectChannelId = (_: RootState, channelId: number) => channelId;

export const selectUserById = selectById;
export const selectUsersByChannelId = createSelector(
  selectEntities,
  selectChannelById,
  selectChannelId,
  (users, channel, channelId) => {
    if (channel === undefined) {
      return [];
    }

    return channel.members
      .filter((member) => member.id in users)
      .map((member) => users[member.id]);
  },
);

export default usersSlice.reducer;
