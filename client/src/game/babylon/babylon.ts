import * as Babylon from "@babylonjs/core";
import { BabylonEvents, createEvents } from "./events";

/** All basic BabylonJS's game elements combined into the one object  */
export interface BabylonWorld {
  canvas: HTMLCanvasElement,
  engine: Babylon.Engine,
  scene: Babylon.Scene,
  events: BabylonEvents,
  camera: Babylon.FreeCamera,
  light: Babylon.HemisphericLight,
}

/** Creates a new instance of Babylon Engine (WebGPU or WebGL2 if user's browser doesn't support WebGPU), scene, scene event manager, camera and light */
export async function createBabylonWorld(): Promise<BabylonWorld> {
  const canvas = document.querySelector<HTMLCanvasElement>("#game");
  let engine: Babylon.Engine;
  
  try {
    engine = new Babylon.WebGPUEngine(canvas);
    await (engine as Babylon.WebGPUEngine).initAsync();
  }
  catch {
    engine = new Babylon.Engine(canvas);
  }

  const scene = new Babylon.Scene(engine);
  const camera = new Babylon.FreeCamera("FreeCamera", new Babylon.Vector3(0, 0, 0), scene);
  const light = new Babylon.HemisphericLight("light1", new Babylon.Vector3(1, 1, 0), scene);

  return {
    canvas,
    engine,
    camera,
    scene,
    light,
    events: createEvents(scene),
  }
}

export * as Babylon from "@babylonjs/core";