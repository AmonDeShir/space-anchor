import { TransformNode } from "@babylonjs/core";
import { BabylonWorld } from "../babylon/babylon"
import { Admiral } from "../admiral/admiral";
import { Store } from "../../store/store";

export type Player = {
  id: number,
  node: TransformNode,
  admiral: Admiral,
}

/**
 * Converts an admiral in to player instance.
 * It also loads global player information.
 * 
 * # Arguments
 * * `admiral` - entity that will be converted into the player instance.
 */
export function createPlayer(admiral: Admiral): Player {
  Store.setPlayer({
    id: admiral.id,
    fraction: admiral.fraction,
    name: admiral.name,
    race: admiral.race,
  });

  return {
    id: admiral.id,
    node: admiral.node,
    admiral,
  }
}
