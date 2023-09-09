import { createEntityAdapter, createSlice } from "@reduxjs/toolkit";
import { Channel } from "@/models";
import { channel } from "diagnostics_channel";

const channelsAdapter = createEntityAdapter<Channel>();

const initialState = channelsAdapter.getInitialState({});

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

const { selectAll, selectTotal } = channelsAdapter.getSelectors();

export const selectChannelList = selectAll;
export const selectChannelCount = selectTotal;

export default channelsSlice.reducer;
