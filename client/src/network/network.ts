import { onCleanup } from "solid-js";

export namespace Networking {
  const url = "ws://127.0.0.1:2137";
  let socket: WebSocket = null;

  let events: { [key: string]: ((data: any) => void)[] } = {};

  export function connect() {
    if (socket != null) {
      return;
    }

    socket = new WebSocket(url);

    socket.addEventListener("message", (e) => {
      try {
        let event = JSON.parse(e.data);
        const key = typeof event === "string" ? event : Object.keys(event)[0]; 
        const data = typeof event === "string" ? null : event[key]; 

        console.log(key, data);

        if (events[key]) {
          for (const handler of events[key]) {
            handler(data);
          }
        }
      }
      catch(e) {}
    });
  }

  export function on<T = any>(key: string, handler: (data: T) => void) {
    if (!events[key]) {
      events[key] = [];
    }

    const index = events[key].indexOf(handler);

    if (index == -1) {
      events[key].push(handler);
    }
    else {
      events[key][index] = handler;
    }
  }

  export function removeEvent<T = any>(key: string, handler: (data: T) => void) {
    if (!events[key]) {
      return;
    }

    events[key] = events[key].filter(e => e !== handler);
  }

  export function send(obj: any) {
    socket.send(JSON.stringify(obj));
  }

  export function disconnect() {
    socket.close();
    socket = null;
  }
}

export const createEventHandler = <T>(event: string, handler: (data: T) => void) => {
  Networking.on(event, handler);

  onCleanup(() => {
    Networking.removeEvent(event, handler);
  })
}
