import { BabylonWorld } from "../babylon";
import { Store } from "../../store/store";

export function updateDebugInfo(world: BabylonWorld) {
  Store.setFps(world.engine.getFps());

  return world;
}

