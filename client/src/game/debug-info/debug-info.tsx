import { BabylonWorld } from "../babylon/babylon";
import { Store } from "../../store/store";
import { Inspector } from '@babylonjs/inspector';
import { Networking } from "../../network/network";

export function updateDebugInfo(world: BabylonWorld) {
  Store.setFps(world.engine.getFps());

  return world;
}

export function addInspector(world: BabylonWorld) {
  Inspector.Show(world.scene, { embedMode: true, showExplorer: true, showInspector: true });
  world.scene.debugLayer.hide();

  window.addEventListener("keydown", (ev) => {
    // Shift+O
    if (ev.shiftKey && ev.key === 'O') {
      if (world.scene.debugLayer.isVisible()) {
        document.querySelector<HTMLDivElement>("#inspector-host").style.zIndex = "0";
        world.scene.debugLayer.hide();
      } else {
        world.scene.debugLayer.show({ showExplorer: true, showInspector: true });
        document.querySelector<HTMLDivElement>("#inspector-host").style.zIndex = "20";
      }
    }
  });
}

export function addBackToMainMenuShortcut() {
  window.addEventListener("keydown", (ev) => {
    // Shift+R
    if (ev.shiftKey && ev.key === 'R') {
      if (window.location.pathname !== "/") {
        Networking.disconnect();
        window.location.href = "/";
      }

    }
  });
}