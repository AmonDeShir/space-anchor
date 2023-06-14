import { GridMaterial } from '@babylonjs/materials/grid';
import { Babylon, BabylonWorld } from '../babylon/babylon';
import { addSkybox } from "../skybox/skybox";
import { Networking } from '../../network/network';
import { Admiral } from '../admiral/admiral';
import { Player } from '../player/player';
import { lerp, timeToPercent } from '../math/utils';
import { fetchMapStateAfterLogin, handleMapStateUpdate, handlePlayerSightRadiusUpdate } from './map.network';

/** Single map update received from server */
export type SingleMapState = {
  time: number,
  admirals: {
    id: number,
    pos: [number, number],
    rot: number,
  }[]
}

/** Offset between map updates */
const INTERPOLATION_OFFSET = 100;

/** Creates a new game map instance */
export function addMap(world: BabylonWorld) {
  const mapState: SingleMapState[] = [];

  const map = createMapMesh(world);
  const centerAxis = new Babylon.AxesViewer(world.scene);

  let admirals: { [key: number]: Admiral } = {};
  let player: Player;

  addSkybox(world, "skybox/galaxy2/galaxy");
  
  //Network events
  fetchMapStateAfterLogin(admirals);
  handlePlayerSightRadiusUpdate(world, admirals, (setter) => { player = setter(player); });
  handleMapStateUpdate(mapState);

  // Mesh events
  movePlayerToClickedMapPoint(world, map);

  // Render loop
  world.scene.registerBeforeRender(() => {
    removeNotVisibleAdmirals(admirals);
    interpolateMapState(mapState, admirals);
  });
}

/** Creates a new plane with a grid material */
function createMapMesh(world: BabylonWorld) {  
  const mesh = Babylon.MeshBuilder.CreatePlane("plane", { size: 2000, sideOrientation: 0  },  world.scene);
  mesh.rotation.x += Math.PI / 2;
  mesh.position.y -= 0.5;
  mesh.material = new GridMaterial("map_material", world.scene); 
  mesh.visibility = 0.5;

  return mesh;
}

/**
 * Moves the player to the clicked point on the map.
 *
 * ## Arguments
 * * `world` - The BabylonWorld object representing the 3D world.
 * * `map` - The mesh representing the map.
 */
function movePlayerToClickedMapPoint(world: BabylonWorld, map: Babylon.Mesh) {
  world.events.on("onPointerDown", (ev, pickResult) => {
    if (ev.button == 0 && pickResult.hit && pickResult.pickedMesh === map) {
      const pos = pickResult.pickedPoint;
      Networking.send({ MoveTo: [pos.x, pos.z] });
    }
  });
}

/** Removes admirals that are not signed by the player. */
function removeNotVisibleAdmirals(admirals: { [key: number]: Admiral }) {
  for (const key of Object.keys(admirals) as unknown as number[]) {
    if (admirals[key]?.state!.get() === "readyToDelete") {
      admirals[key].dispose();
      delete admirals[key];
    }
  }
}

/** Interpolates between received map states to smoothly update the map. */
function interpolateMapState(states: SingleMapState[], admirals: { [key: number]: Admiral }) {
  if (states.length < 2) {
    return;
  }

  const renderTime = Date.now() - INTERPOLATION_OFFSET;

  while (states.length > 2 && renderTime > states[1].time) {
    states.shift();
  }

  const interpolationFactor = timeToPercent(renderTime, states[0].time, states[1].time);

  interpolateAdmiralsTransform(admirals, states[1], interpolationFactor);
}

/**
 * Interpolates the transforms of admirals to reach the actual map state sent by the server.
 *
 * ## Arguments
 * 
 * * `admirals` - An object containing the admirals to be interpolated.
 * * `state` - A object representing the map state to interpolate.
 * * `interpolationFactor` - A number between 0 and 1 representing the interpolation factor.
 */
function interpolateAdmiralsTransform(admirals: { [key: number]: Admiral }, state: SingleMapState, interpolationFactor: number) {
  for (let packet of state.admirals) {
    const admiral = admirals[packet.id];

    if (!admiral) {
      continue
    }

    changeAdmiralAnimations(admiral, packet.pos);

    admiral.node.position.x = lerp(admiral.node.position.x, packet.pos[0], interpolationFactor);
    admiral.node.position.z = lerp(admiral.node.position.z, packet.pos[1], interpolationFactor);
    admiral.mesh.rotation.y = lerp(admiral.mesh.rotation.y, packet.rot, interpolationFactor);
  }
}

/** Changes the animations of an admiral based on the feature position. */
function changeAdmiralAnimations(admiral: Admiral, featurePosition: [number, number]) {
  const deltaMove = Math.max(Math.abs(admiral.node.position.x - featurePosition[0]), Math.abs(admiral.node.position.z - featurePosition[1]));

  if (admiral.state.get() === "idle" && deltaMove >= 0.01) {
    admiral.state.set("move");
  }

  if (admiral.state.get() === "move" && deltaMove < 0.001) {
    admiral.state.set("idle");
  }
}