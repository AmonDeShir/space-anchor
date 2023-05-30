import { BabylonWorld } from "../babylon";
import { Store } from "../../store/store";
import { Inspector } from '@babylonjs/inspector';

export function updateDebugInfo(world: BabylonWorld) {
  Store.setFps(world.engine.getFps());

  return world;
}

export function addInspctor(world: BabylonWorld) {
  Inspector.Show(world.scene, {});
  world.scene.debugLayer.hide();

  window.addEventListener("keydown", (ev) => {
    // Shift+Ctrl+Alt+I
    if (ev.shiftKey &&  ev.key === 'I') {
      if (world.scene.debugLayer.isVisible()) {
        world.scene.debugLayer.hide();
      } else {
        world.scene.debugLayer.show();
      }
    }
  });
}