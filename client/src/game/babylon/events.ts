import { Babylon } from "./babylon";

/** Type representing the Babylon.js events. */
export type BabylonEvents = {
  /**
   * Adds an event listener for the specified event.
   * @param event - The event to listen for.
   * @param callback - The callback function to execute when the event is triggered.
   */
  on: <T extends Event>(event: T, callback: Callback[T]) => void,
  /**
   * Removes an event listener for the specified event.
   * @param event - The event to remove the listener from.
   * @param callback - The callback function to remove.
   */
  remove: <T extends Event>(event: T, callback: Callback[T]) => void,
};

/** Type representing the available events. */
export type Event = keyof Callback;

/** Type representing the callback functions for each event. */
export type Callback = {
  "onPointerDown": (ev: Babylon.IPointerEvent, pickResult: Babylon.PickingInfo) => void,
  "onPointerUp": (ev: Babylon.IPointerEvent, pickResult: Babylon.PickingInfo) => void,
  "onPointerMove": (ev: Babylon.IPointerEvent, pickResult: Babylon.PickingInfo) => void,
}

/** Array version of the Callback type */
export type CallbackMap = {
  [K in Event]: Callback[K][];
}

/** Creates new instance of event controller for the BabylonJS's scene */
export function createEvents (scene: Babylon.Scene) {
  const events: CallbackMap = {
    "onPointerDown": [],
    "onPointerUp": [],
    "onPointerMove": [],
  };

  scene.onPointerDown = (ev, pickInfo) => events["onPointerDown"].map(callback => callback(ev, pickInfo));
  scene.onPointerUp = (ev, pickInfo) => events["onPointerUp"].map(callback => callback(ev, pickInfo));
  scene.onPointerMove = (ev, pickInfo) => events["onPointerMove"].map(callback => callback(ev, pickInfo));

  return {
    on: <T extends Event>(event: T, callback: Callback[T]) => {
      events[event].push(callback);
    },

    remove: <T extends Event>(event: T, callback: Callback[T]) => {
      events[event] = events[event].filter(e => e !== callback);
    },
  }
}
