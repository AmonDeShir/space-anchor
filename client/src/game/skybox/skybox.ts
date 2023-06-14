import * as Babylon from "@babylonjs/core";
import { BabylonWorld } from "../babylon/babylon";

/** 
 * Creates an skybox and adds it to the scene.
 *
 * ## Arguments
 * * `world` - babylon instance
 * * `texture` - path to the skybox textures. Path root is public folder.
 * 
 * ## Example
 * This example will load textures:
 * * `/public/skybox/test_nx.jpg`
 * * `/public/skybox/test_ny.jpg`
 * * `/public/skybox/test_nz.jpg`
 * * `/public/skybox/test_px.jpg`
 * * `/public/skybox/test_py.jpg`
 * * `/public/skybox/test_pz.jpg`
 * and displays them as a skybox
 * 
 * @example addSkybox(world, "skybox/test")
 */
export function addSkybox(world: BabylonWorld, textures: string) {
  const skybox = Babylon.MeshBuilder.CreateBox("skyBox", { size: 2000.0 }, world.scene);
  const skyboxMaterial = new Babylon.StandardMaterial("skyBox", world.scene);

  skyboxMaterial.backFaceCulling = false;
  skyboxMaterial.reflectionTexture = new Babylon.CubeTexture(textures, world.scene);
  skyboxMaterial.reflectionTexture.coordinatesMode = Babylon.Texture.SKYBOX_MODE;
  skyboxMaterial.diffuseColor = new Babylon.Color3(0, 0, 0);
  skyboxMaterial.specularColor = new Babylon.Color3(0, 0, 0);
  skybox.material = skyboxMaterial;
}