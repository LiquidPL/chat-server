import {
  createEntityAdapter,
  createSelector,
  createSlice,
} from "@reduxjs/toolkit";
import { Channel, ChannelListItem, Message } from "@/models";
import { RootState } from "@/store";
import { selectAllMessages } from "./messages";

const channelsAdapter = createEntityAdapter<Channel>();

const initialState = channelsAdapter.getInitialState();

export const channelsSlice = createSlice({
  name: "channels",
  initialState,
  reducers: {
    setChannel: channelsAdapter.upsertOne,
    setChannels: channelsAdapter.upsertMany,
    deleteChannel: channelsAdapter.removeOne,
  },
});

export const { setChannel, setChannels, deleteChannel } = channelsSlice.actions;

const { selectById, selectAll, selectTotal } = channelsAdapter.getSelectors(
  (state: RootState) => state.channels,
);

export const selectChannelList = createSelector(
  selectAll,
  selectAllMessages,
  (channels, messages) => {
    return channels
      .map((channel) => {
        let mostRecentMessage: Message | undefined;

        if (messages.length > 0) {
          let channelMessages = messages.filter(
            (message) => message.channel_id == channel.id,
          );

          if (channelMessages.length > 0) {
            mostRecentMessage = channelMessages.reduce((largest, current) => {
              const largestDate = new Date(largest.created_at);
              const currentDate = new Date(current.created_at);

              return currentDate > largestDate ? current : largest;
            });
          }
        }

        return {
          channel,
          mostRecentMessage,
        } as ChannelListItem;
      })
      .sort((a, b) => {
        return (
          new Date(
            b.mostRecentMessage?.created_at ?? b.channel.created_at,
          ).getTime() -
          new Date(
            a.mostRecentMessage?.created_at ?? a.channel.created_at,
          ).getTime()
        );
      });
  },
);

export const selectChannelById = selectById;
export const selectChannelCount = selectTotal;

export default channelsSlice.reducer;
