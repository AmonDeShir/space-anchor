import { createBabylonWorld } from "./babylon/babylon";
import { addBackToMainMenuShortcut, addInspector, updateDebugInfo } from "./debug-info/debug-info";
import { createEffect } from "solid-js";
import { Store } from "../store/store";
import { addMap } from "./map/map";
import { createCameraController } from "./camera/camera";

export const DEBUG_MOD = true;

/** Creates a new game instance */ 
export async function createGame() {
  const world = await createBabylonWorld();

  createCameraController(world);
  addMap(world);

  if (DEBUG_MOD) {
    addInspector(world);
    addBackToMainMenuShortcut();  
  }

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