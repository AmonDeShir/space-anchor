import { createStore } from "solid-js/store";
import { DebugInfo, PlayerInfo, Settings } from "./store.types";

export namespace Store {
  export const [debugInfo, setDebugInfo] = createStore<DebugInfo>({ fps: 0, });
  export const [other, setOther] = createStore({ shouldRender: false, });
  export const [player, setPlayer] = createStore<PlayerInfo>({ fraction: "", id: -1, name: "", race: "" });
  export const [settings, setSettings] = createStore<Settings>({
    edgeScrolling: false,
    edgeScrollingRotationSpeed: 0.003,
    edgeScrollingZoomSpeed: 0.0006,
    zoomSpeed: 10,
  });

  
  export const setFps = (value: number) => setDebugInfo("fps", value); 
}