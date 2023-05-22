import { pipe } from "bitecs";
import { BabylonWorld } from "../babylon";
import { Store } from "../../store/store";

function updateDebugInfo(world: BabylonWorld) {
  Store.setFps(world.engine.getFps());

  return world;
}

export const debugInfoPipeline = pipe(updateDebugInfo);