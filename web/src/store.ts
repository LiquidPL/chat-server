import { configureStore } from "@reduxjs/toolkit";

import userReducer from "./state/user";
import channelsReducer from "./state/channels";
import messagesReducer from "./state/messages";

const store = configureStore({
  reducer: {
    user: userReducer,
    channels: channelsReducer,
    messages: messagesReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export default store;
