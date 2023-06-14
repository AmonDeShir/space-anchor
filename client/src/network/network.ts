import { onCleanup } from "solid-js";

export namespace Networking {
  const url = "ws://127.0.0.1:2137";
  let socket: WebSocket = null;

  let events: { [key: string]: ((data: any) => void)[] } = {};

  /** Connects to the WebSocket server. */
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

        if (events[key]) {
          for (const handler of events[key]) {
            handler(data);
          }
        }
      }
      catch(e) {}
    });
  }

  /**
   * Registers an event listener for the specified message key.
   * 
   * ## Arguments
   * 
   * * `key` - The key of the event message.
   * * `handler` - The handler function to be called when the event occurs.
   * * `T` - The type of data expected as the event argument. 
   *
   */
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

  /**
   * Removes an event listener for the specified message key.
   * 
   * ## Arguments
   * 
   * * `key` - The key of the event message.
   * * `handler` - The handler function to be called when the event occurs.
   * * `T` - The type of data expected as the event argument. 
   */
  export function removeEvent<T = any>(key: string, handler: (data: T) => void) {
    if (!events[key]) {
      return;
    }

    events[key] = events[key].filter(e => e !== handler);
  }


  /**
   * Sends data to the WebSocket server.
   * 
   * ## Arguments
   * 
   * @param obj - the object of a message to send.
   */
  export function send(obj: any) {
    socket?.send(JSON.stringify(obj));
  }

  /** Disconnects from the WebSocket server. */
  export function disconnect() {
    socket?.close();
    socket = null;
  }
}

/**
 * Solid-js hook for listening to a server event. The event listener will be automatically deleted when the SolidJS component is unmounted.
 * 
 * ## Arguments
 * 
 * * `event` - The key of the event message.
 * * `handler` - The handler function to be called when the event occurs.
 * * `T` - The type of data expected as the event argument. 
 */
export const createEventHandler = <T>(event: string, handler: (data: T) => void) => {
  Networking.on(event, handler);

  onCleanup(() => {
    Networking.removeEvent(event, handler);
  })
}
