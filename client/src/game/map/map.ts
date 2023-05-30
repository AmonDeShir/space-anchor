import { GridMaterial } from '@babylonjs/materials/grid';
import { Babylon, BabylonWorld } from '../babylon';
import { addSkybox } from "../skybox";


function createMapMesh(world: BabylonWorld) {
  const mesh = Babylon.MeshBuilder.CreatePlane("plane", { size: 2000, sideOrientation: 0  },  world.scene);
  mesh.rotation.x += Math.PI / 2;
  mesh.position.y -= 0.5;
  mesh.material = new GridMaterial("map_material", world.scene); 
  mesh.visibility = 0.5;

  return mesh;
}


export function addMap(world: BabylonWorld) {
  const map = createMapMesh(world);
  const axes = new Babylon.AxesViewer(world.scene)

  addSkybox(world, "skybox/galaxy2/galaxy");
}