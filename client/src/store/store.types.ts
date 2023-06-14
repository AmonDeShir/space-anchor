import { float } from "@babylonjs/core"

export type DebugInfo = {
  fps: number,
}

export type PlayerInfo = {
  race: string,
  name: string,
  fraction: string,
  id: number,
}

export type Settings = {
  edgeScrolling: boolean,
  edgeScrollingRotationSpeed: float,
  edgeScrollingZoomSpeed: float,
  zoomSpeed: float,  
}