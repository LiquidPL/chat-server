import { configureStore } from "@reduxjs/toolkit";

import userReducer from "./state/user";
import channelsReducer from "./state/channels";

const store = configureStore({
  reducer: {
    user: userReducer,
    channels: channelsReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export default store;
