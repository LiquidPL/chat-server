import { Message } from "@/models";
import { RootState } from "@/store";
import {
  createEntityAdapter,
  createSelector,
  createSlice,
} from "@reduxjs/toolkit";

const messagesAdapter = createEntityAdapter<Message>();

const initialState = messagesAdapter.getInitialState({});

export const messagesSlice = createSlice({
  name: "messages",
  initialState,
  reducers: {
    addMessage: messagesAdapter.upsertOne,
    addMessages: messagesAdapter.upsertMany,
    deleteMessage: messagesAdapter.removeOne,
  },
});

export const { addMessage, addMessages, deleteMessage } = messagesSlice.actions;

const { selectAll } = messagesAdapter.getSelectors(
  (state: RootState) => state.messages,
);

const selectChannelId = (_: RootState, channelId: number) => channelId;

export const selectMessagesByChannelId = createSelector(
  [selectAll, selectChannelId],
  (messages, channelId) => {
    return messages
      .filter((message) => message.channel_id == channelId)
      .sort((a, b) => {
        return (
          new Date(a.created_at).getTime() - new Date(b.created_at).getTime()
        );
      });
  },
);

export const selectAllMessages = selectAll;

export default messagesSlice.reducer;
