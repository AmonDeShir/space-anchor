import { createStore } from "solid-js/store";
import { DebugInfo } from "./store.types";

export namespace Store {
  export const [debugInfo, setDebugInfo] = createStore<DebugInfo>({ fps: 0, });
  export const [other, setOther] = createStore({ shouldRender: false, });
  
  
  export const setFps = (value: number) => setDebugInfo("fps", value); 
}