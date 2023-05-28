import * as Babylon from "@babylonjs/core";
import { BabylonWorld } from "./babylon";
import { addStaticMeshComponent } from "./mesh/mesh";
import { addComponent, addEntity, defineComponent } from "bitecs";

export const Skybox = defineComponent();

export function addSkybox(world: BabylonWorld, textures: string) {
  const skybox = Babylon.MeshBuilder.CreateBox("skyBox", { size: 2000.0 }, world.scene);
  const skyboxMaterial = new Babylon.StandardMaterial("skyBox", world.scene);

  skyboxMaterial.backFaceCulling = false;
  skyboxMaterial.reflectionTexture = new Babylon.CubeTexture(textures, world.scene);
  skyboxMaterial.reflectionTexture.coordinatesMode = Babylon.Texture.SKYBOX_MODE;
  skyboxMaterial.diffuseColor = new Babylon.Color3(0, 0, 0);
  skyboxMaterial.specularColor = new Babylon.Color3(0, 0, 0);
  skybox.material = skyboxMaterial;

  const box = addEntity(world);
  addComponent(world, Skybox, box);
  addStaticMeshComponent(box, skybox);
}