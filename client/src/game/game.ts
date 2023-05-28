import { createWorld, pipe, addEntity } from "bitecs";
import { BabylonWorld, createBabylonWorld } from "./babylon";
import { Rotation, addMeshComponent, meshPipeline, meshQuery } from "./mesh/mesh";
import * as Babylon from "@babylonjs/core";
import { Inspector } from '@babylonjs/inspector';
import { debugInfoPipeline } from "./debug-info/debug-info";
import { createEffect } from "solid-js";
import { Store } from "../store/store";
import { addSkybox } from "./skybox";

export const world = createWorld(createBabylonWorld());
 
function rotateBox(world: BabylonWorld) {
  const eids = meshQuery(world); 
  let i = -1;

  for (const id of eids) {
    Rotation.y[id] += 0.1 * i  * world.engine.getDeltaTime(); 
    i *= -1;
  }

  return world;
}

export function createGame(world: BabylonWorld) {
  const pipeline = pipe(meshPipeline, rotateBox, debugInfoPipeline);

  addSkybox(world, "skybox/galaxy2/galaxy");

  const box = addEntity(world);
  addMeshComponent(box, Babylon.MeshBuilder.CreateBox("box1", { width: 0.5, height: 0.5, depth: 0.5 }, world.scene));


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


  const renderLoop = () => {
    pipeline(world);
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