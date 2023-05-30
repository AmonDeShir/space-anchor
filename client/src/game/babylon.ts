import * as Babylon from "@babylonjs/core";

export interface BabylonWorld {
  canvas: HTMLCanvasElement,
  engine: Babylon.Engine,
  scene: Babylon.Scene,
  camera: Babylon.UniversalCamera,
  light: Babylon.HemisphericLight,
}

export async function createBabylonWorld(): Promise<BabylonWorld> {
  const canvas = document.querySelector<HTMLCanvasElement>("#game");
  const engine = new Babylon.WebGPUEngine(canvas);
  await engine.initAsync();

  const scene = new Babylon.Scene(engine);
  const camera = new Babylon.UniversalCamera("UniversalCamera", new Babylon.Vector3(0, 0, -10), scene);
  const light = new Babylon.HemisphericLight("light1", new Babylon.Vector3(1, 1, 0), scene);

  camera.setTarget(Babylon.Vector3.Zero());
  camera.attachControl(canvas, true);

  return {
    canvas,
    engine,
    camera,
    scene,
    light,
  }
}

export * as Babylon from "@babylonjs/core";