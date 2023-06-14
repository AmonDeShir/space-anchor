import { createEffect, createSignal } from "solid-js";
import { Babylon, BabylonWorld } from "../babylon/babylon";
import * as GUI from "@babylonjs/gui";
import { Mesh, TransformNode } from "@babylonjs/core";


export type EntityDescription = {
  node: TransformNode,
  mesh: Mesh,
  label: GUI.TextBlock,
  setText: (text: string) => void,
}

type Props = {
  /** Displayed text. Default: "" */
  text?: string;
  /** Text's font family. All fonts available in CSS can be used. Default: "Fervojo" */
  font?: string;
  /** Size of displayed text. Default: 200 */
  fontSize?: number;
  /** Color of displayed text. Default: "#ff7b00" */
  color?: string;
  /** Size of text container [width, height]. Default: [2, 1] */
  meshSize?: [number, number];
  /** Margin of the text from the followed object. Default: 0.5 */
  radius?: number;
};

/**
 * Represents an entity description that follows a specific object and orbits around it, always facing the camera.
 *
 * ## Arguments
 * 
 * * `props` - The properties for the text.
 * * `world` - The BabylonWorld object representing the 3D world.
*/
export function spawnEntityDescription(entity: Babylon.Node, props: Props, world: BabylonWorld): EntityDescription {
  if (!props.meshSize) {
    props.meshSize = [2, 1];
  }

  const node = new Babylon.TransformNode(entity.name + " description", world.scene);
  const mesh = Babylon.MeshBuilder.CreatePlane("text-mesh", { width: props.meshSize[0] ?? 2, height: props.meshSize[1]?? 1 }, world.scene);
  const [text, setText] = createSignal(props.text ?? "");

  node.setParent(entity);
  node.position.set(0, 0, 0); 

  mesh.setParent(node);
  mesh.position.z = props.radius ?? 0.5;
  mesh.position.y = -1.0;
  mesh.rotation.y = Math.PI;

  const advancedTexture = GUI.AdvancedDynamicTexture.CreateForMesh(mesh);

  const label = new GUI.TextBlock("text", text());
  label.width = 1;
  label.height = 1;
  label.color = props.color ?? "#ff7b00";
  label.fontSize = props.fontSize ?? 50;
  label.fontFamily = props.font ?? "Fervojo";

  advancedTexture.addControl(label);

  world.scene.registerBeforeRender(() => {
    const pivotRotation = node.rotation.clone();

    const parent = node.parent as TransformNode;
    let parentRot = parent?.rotation.y ?? 0;

    node.lookAt(world.camera.globalPosition);
    node.rotation.x = pivotRotation.x;
    node.rotation.z = pivotRotation.z;
    node.rotation.y -= parentRot;

    mesh.rotation.x = world.camera.rotation.x;
  });

  createEffect(() => {
    label.text = text();
  });

  return {
    node,
    mesh,
    label,
    setText,
  }
}