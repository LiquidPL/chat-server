import { configureStore } from "@reduxjs/toolkit";

import authReducer from "./state/auth";
import channelsReducer from "./state/channels";
import messagesReducer from "./state/messages";
import usersReducer from "./state/users";

const store = configureStore({
  reducer: {
    auth: authReducer,
    channels: channelsReducer,
    messages: messagesReducer,
    users: usersReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export default store;
