import { Message } from "@/models";
import { EntityState, createEntityAdapter, createSelector, createSlice } from "@reduxjs/toolkit";

const messagesAdapter = createEntityAdapter<Message>();

const initialState = messagesAdapter.getInitialState({});

export const messagesSlice = createSlice({
  name: "messages",
  initialState,
  reducers: {
    addMessage: messagesAdapter.upsertOne,
    addMessages: messagesAdapter.upsertMany,
    deleteMessage: messagesAdapter.removeOne,
  }
});

export const { addMessage, addMessages, deleteMessage } = messagesSlice.actions;

const { selectAll } = messagesAdapter.getSelectors();

const selectChannelId = (_: EntityState<Message>, channelId: number) => channelId;

export const selectMessagesByChannelId = createSelector([selectAll, selectChannelId], (messages, channelId) => {
  return messages
    .sort((a, b) => {
      return new Date(a.created_at).getTime() - new Date(b.created_at).getTime()
    })
    .filter((message) => message.channel_id == channelId);
});

export const selectLastMessageByChannelId = createSelector([selectMessagesByChannelId, selectChannelId], (messages, _) => {
  return messages[messages.length - 1];
})

export default messagesSlice.reducer;
