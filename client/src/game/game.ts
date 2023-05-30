import { BabylonWorld, createBabylonWorld } from "./babylon";
import * as Babylon from "@babylonjs/core";
import { addInspctor, updateDebugInfo } from "./debug-info/debug-info";
import { createEffect } from "solid-js";
import { Store } from "../store/store";
import { addMap } from "./map/map";

export let world: BabylonWorld;
 

export async function createGame() {
  world = await createBabylonWorld();

  let box = Babylon.MeshBuilder.CreateBox("box1", { width: 0.5, height: 0.5, depth: 0.5 }, world.scene);

  addMap(world);
  addInspctor(world);

  const renderLoop = () => {
    updateDebugInfo(world);
    world.scene.render();
  };

  createEffect(() => {
    if (Store.other.shouldRender) {
      world.engine.runRenderLoop(renderLoop);
    }
    else {
      world.engine.stopRenderLoop(renderLoop);
    }
  });
}