import { CHANNEL_DELETED } from "@/app/channels/[id]/page";
import getConfig from "@/config";
import {
  Event,
  Auth,
  UserAuthenticated,
  MessageCreated,
  MessageDeleted,
  ChannelCreated,
  ChannelDeleted,
} from "@/models";
import { deleteChannel, setChannel, setChannels } from "@/state/channels";
import { addMessage, deleteMessage } from "@/state/messages";
import store from "@/store";

let socket: WebSocket;

function startWebSocket() {
  const socketUrl = getConfig().apiUrl.replace("http", "ws") + "/websocket";
  socket = new WebSocket(socketUrl);

  socket.addEventListener("open", onConnect);
  socket.addEventListener("message", onMessage);
}

function buildPayload(accessToken: string): Event<Auth> {
  return {
    event_type: "Auth",
    data: { token: accessToken },
  };
}

function onConnect() {
  let accessToken = store.getState().user.accessToken;

  if (accessToken !== undefined) {
    socket.send(JSON.stringify(buildPayload(accessToken)));
  }
}

function onMessage(message: MessageEvent<any>) {
  const event = JSON.parse(message.data) as Event<any>;

  handleEvent(event);
}

function handleEvent(event: Event<any>) {
  switch (event.event_type) {
    case "UserAuthenticated":
      const initial_channels = (event.data as UserAuthenticated).channels;

      for (const initial_channel of initial_channels) {
        store.dispatch(setChannel(initial_channel.channel));

        if (initial_channel.message !== null) {
          store.dispatch(addMessage(initial_channel.message));
        }
      }

      break;
    case "MessageCreated":
      const message = event.data as MessageCreated;
      store.dispatch(addMessage(message));
      break;
    case "MessageDeleted":
      const messageId = (event.data as MessageDeleted).id;
      store.dispatch(deleteMessage(messageId));
      break;
    case "ChannelCreated":
      const channel = event.data as ChannelCreated;
      store.dispatch(setChannel(channel));
      break;
    case "ChannelDeleted":
      const channelId = (event.data as ChannelDeleted).id;
      store.dispatch(deleteChannel(channelId));
      PubSub.publish(CHANNEL_DELETED, channelId);
      break;
    default:
      break;
  }
}

export const webSocketService = {
  startWebSocket,
};
