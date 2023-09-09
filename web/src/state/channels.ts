import { createSelector, createSlice, PayloadAction } from "@reduxjs/toolkit";
import { Channel } from "@/models";
import { RootState } from "@/store";

export interface ChannelState {
  [key: number]: Channel;
}

const initialState: ChannelState = {};

export const channelsSlice = createSlice({
  name: "channels",
  initialState,
  reducers: {
    setChannel: (state, action: PayloadAction<Channel>) => {
      state[action.payload.id] = action.payload;
    },
    deleteChannel: (state, action: PayloadAction<number>) => {
      if (!(action.payload in state)) {
        return;
      }

      delete state[action.payload];
    },
  },
});

export const { setChannel: createChannel, deleteChannel } =
  channelsSlice.actions;

const selectChannels = (state: RootState) => state.channels;

export const selectChannelList = createSelector(selectChannels, (channels) => {
  return Object.entries(channels).reduce((acc, [_, value]) => {
    acc.push(value);
    return acc;
  }, [] as Channel[]);
});


export const selectChannelCount = createSelector(selectChannels, (channels) => {
  return Object.keys(channels).length;
})

export default channelsSlice.reducer;
