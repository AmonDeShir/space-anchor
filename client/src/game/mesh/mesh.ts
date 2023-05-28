import { Types, addComponent, defineComponent, defineQuery, pipe } from "bitecs";
import { BabylonWorld, Babylon } from "../babylon";
import { world } from "../game";

export const Vector3 = { x: Types.f32, y: Types.f32, z: Types.f32 }
export const Mesh = defineComponent();
export const Position = defineComponent(Vector3);
export const Rotation = defineComponent(Vector3);

export const meshQuery = defineQuery([Position, Rotation, Mesh]);
export const allMeshQuery = defineQuery([Mesh]);

export function addMeshComponent(eid: number, mesh: Babylon.Mesh, pos = Babylon.Vector3.Zero(), rot = Babylon.Quaternion.Zero()) {
  addComponent(world, Mesh, eid);
  addComponent(world, Position, eid);
  addComponent(world, Rotation, eid);

  Position.x[eid] = pos.x;
  Position.y[eid] = pos.y;
  Position.z[eid] = pos.z;

  Rotation.x[eid] = rot.x;
  Rotation.y[eid] = rot.y;
  Rotation.z[eid] = rot.z;

  world.objects[eid] = mesh;
}

export function addStaticMeshComponent(eid: number, mesh: Babylon.Mesh) {
  addComponent(world, Mesh, eid);
  world.objects[eid] = mesh;
}

function findUnusedKeys(elements: { [key: number]: Babylon.Mesh }, actual: number[]) {
  const unused = {...elements};
  
  for (const eid of actual) {
    delete unused[eid];
  }

  return unused;
}

export function meshDispatcherSystem(world: BabylonWorld) {
  const unused = findUnusedKeys(world.objects, allMeshQuery(world));

  for (const eid of Object.keys(unused) as unknown as number[]) {
    world.objects[eid].dispose();
    delete world.objects[eid];
  }

  return world;
}

export function meshRenderingSystem(world: BabylonWorld) {
  const eids = meshQuery(world);

  for (const eid of eids) {
    const mesh = world.objects[eid];

    mesh.position.x = Position.x[eid];
    mesh.position.y = Position.y[eid];
    mesh.position.z = Position.z[eid];

    mesh.rotation.x = Rotation.x[eid];
    mesh.rotation.y = Rotation.y[eid];
    mesh.rotation.z = Rotation.z[eid];
  }

  return world;
}


export const meshPipeline = pipe(meshDispatcherSystem, meshRenderingSystem);