import getConfig from "@/config";
import { Event, Auth, UserAuthenticated, MessageCreated } from "@/models";
import { setChannels } from "@/state/channels";
import { addMessage } from "@/state/messages";
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
      const channels = (event.data as UserAuthenticated).channels;
      store.dispatch(setChannels(channels));
      break;
    case "MessageCreated":
      const message = (event.data as MessageCreated);
      store.dispatch(addMessage(message));
    case "MessageCreated":
    default:
      break;
  }
}

export const webSocketService = {
  startWebSocket,
};
