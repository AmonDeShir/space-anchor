import { TransformNode } from "@babylonjs/core";
import { Babylon, BabylonWorld } from "../babylon/babylon"
import { spawnEntityDescription } from "../entity-description/entity-description";
import { AdmiralFiniteStateMachine, createAdmiralFiniteStateMachine } from "./admiral.states";
import { AnimationManager, createAnimationManager } from "../animations/animations";

export type Admiral = {
  id: number,
  mesh: Babylon.AbstractMesh,
  name: string,
  fraction: string,
  race: string,
  node: TransformNode,
  state: AdmiralFiniteStateMachine,
  animations: AnimationManager,

  /** Unregisters all event listeners related to the Admiral */
  dispose: () => void;
}

/** Type definition for the Admiral packet. This packet is received from server as argument of the AdmiralAppears message */
export type AdmiralPacket = {
  id: number,
  fraction: string,
  name: string,
  pos: [number, number],
  race: string,
  rotation: number,
}

/** Type definition for the AdmiralsPosition packet. This packet is received from server as argument of the PositionOfAdmiralsInSign message 
 * 
 * [`number` - id of the admiral, [`number` - position x, `number` - position y], `number` - rotation in radians]
*/
export type AdmiralsPosition = [number, [number, number], number];

/**
 * Function that spawns an admiral in the Babylon world.
 * 
 * It will load the admiral model, prepare finite state machine and create animation manager. 
 * 
 * ## Arguments
 * 
 * @param admiral - The Admiral packet containing admiral details.
 * @param world - The Babylon world instance.
 * @returns 
 */
export async function spawnAdmiral(admiral: AdmiralPacket, world: BabylonWorld): Promise<Admiral> {
  const transform = new Babylon.TransformNode(`Admiral ${admiral.name}`, world.scene);
  const model = await Babylon.SceneLoader.ImportMeshAsync("", "./models/", "base_ship.gltf", world.scene);
  const ship = model.meshes[0];

  ship.setParent(transform);
  ship.rotation = new Babylon.Vector3(0, 0, 0);

  spawnEntityDescription(transform, { text: admiral.name }, world);
  
  transform.position.x = admiral.pos[0];
  transform.position.y = 0.5;
  transform.position.z = admiral.pos[1];
  transform.rotation.y = admiral.rotation;

  const animations = createAnimationManager(world, {
    "appear": model.animationGroups[0],
    "disappear": model.animationGroups[1],
    "idle": model.animationGroups[2],
    "move": model.animationGroups[3],
  });
  
  const state = createAdmiralFiniteStateMachine(animations, world);

  return {
    id: admiral.id,
    mesh: ship,
    name: admiral.name,
    race: admiral.race,
    fraction: admiral.fraction,
    node: transform,
    animations,
    state,
    dispose: () => {
      transform.dispose();
      animations.dispose();
      state.dispose();
      
      model.animationGroups.map(item => item.dispose());
      model.geometries.map(item => item.dispose());
      model.lights.map(item => item.dispose());
      model.meshes.map(item => item.dispose());
      model.particleSystems.map(item => item.dispose());
      model.skeletons.map(item => item.dispose());
      model.transformNodes.map(item => item.dispose());
    }
  }
}
