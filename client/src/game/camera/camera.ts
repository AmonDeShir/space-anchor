import { Camera, FreeCamera, TransformNode, Vector3 } from "@babylonjs/core";
import { Babylon, BabylonWorld } from "../babylon/babylon";
import { Store } from "../../store/store";
import { circularArcInterpolation, clamp, lerp } from "../math/utils";

/** Creates a camera controller for the 3D scene, enabling camera rotation, zoom, and movement. */
export function createCameraController(world: BabylonWorld) {
  const camera = initializeCamera(world);
  const pivot = createPivot(world, camera);

  let rightClick = false;
  let initialMouseX = 0;
  let initialRotationY = 0;
  const moveAcceleration = 5;
  const maxMovieSpeedRange: [number, number] = [10, 50];
  let currentMaxMoveSpeed = 20;
  let currentSpeed = 0;
  const maxZoomVelocity = 0.2;
  const minZoomVelocity = -0.2;
  let zoomVelocity = 0;
  let currentZoom = 0;
  let mouseX = document.body.clientWidth / 2;
  let mouseY = document.body.clientHeight / 2;

  const keyStates: { [key: string]: boolean } = {};

  document.addEventListener("keydown", (e) => {
    keyStates[e.code] = true;
  });

  document.addEventListener("keyup", (e) => {
    keyStates[e.code] = false;
  });

  world.events.on("onPointerDown", (e) => {
    if (e.button === 2) {
      rightClick = true;
      initialMouseX = e.clientX;
      initialRotationY = pivot.rotation.y;
    }
  });

  world.events.on("onPointerUp", (e) => {
    if (e.button === 2) {
      rightClick = false;
    }
  });

  world.events.on("onPointerMove", (e) => {
    mouseX = e.clientX;
    mouseY = e.clientY;
    
    if (rightClick) {
      rotateCamera(pivot, mouseX, initialMouseX, initialRotationY);
    }
  });

  document.addEventListener("wheel", (event) => {
    const zoomDelta = Math.sign(event.deltaY) * 0.01 * world.engine.getDeltaTime();
    zoomVelocity += zoomDelta;
    zoomVelocity = clamp(zoomVelocity, minZoomVelocity, maxZoomVelocity);
  });

  world.scene.registerBeforeRender(() => {
    const deltaTime = Math.min(world.engine.getDeltaTime() / 1000, 1);

    currentMaxMoveSpeed = calcMaxMoveSpeed(currentZoom, maxMovieSpeedRange);
    const direction = calculateDirection(keyStates, pivot);
    
    if (Store.settings.edgeScrolling) {
      edgeBasedMouseRotationControl(mouseX, pivot, world.engine.getDeltaTime());
    }

    const targetSpeed = direction.length() > 0 ? currentMaxMoveSpeed : 0;
    currentSpeed = lerp(currentSpeed, targetSpeed, deltaTime * moveAcceleration);

    const displacement = direction.scale(currentSpeed * deltaTime);
    pivot.position.addInPlace(displacement);

    if (Store.settings.edgeScrolling) {
      zoomVelocity = edgeBasedMouseZoomControl(mouseY, world.engine.getDeltaTime(), zoomVelocity, minZoomVelocity, maxZoomVelocity);
    }

    currentZoom = lerp(currentZoom, currentZoom + zoomVelocity, deltaTime * Store.settings.zoomSpeed);
    currentZoom = clamp(currentZoom, 0, 1);
    
    adjustCameraZoomByArcMovement(camera, currentZoom);
    targetSceneCenter(camera);

    // Slowdown zoom in time
    zoomVelocity *= 0.9;
  });
}

/** Calculates the maximum movement speed of the camera based on the current zoom progress. */
function calcMaxMoveSpeed(zoomProgress: number, speedRange: [number, number]): number {
  const [min, max] = speedRange;

  return lerp(min, max, 1 - zoomProgress);
}

/** Sets optimal camera position and rotation */
function initializeCamera(world: BabylonWorld): FreeCamera {
  const camera = world.camera;
  camera.parent = null;
  camera.position.set(0, 5, -5);
  camera.rotation.set(0, 0.5, 0);
  camera.setTarget(new Vector3(0, 0.5, 0));
  return camera;
}

/** Creates and configures camera pivot */
function createPivot(world: BabylonWorld, camera: Camera) {
  const pivot = new Babylon.TransformNode("camera", world.scene);
  camera.parent = pivot;
  pivot.position.set(0, 0, 0);
  return pivot;
}

/** Rotates the camera around the pivot based on mouse movement. */
function rotateCamera(pivot: Babylon.TransformNode, currentMouseX: number, initialMouseX: number, initialRotationY: number) {
  const deltaX = currentMouseX - initialMouseX;
  const normalizedDeltaX = deltaX / pivot.getScene().getEngine().getRenderWidth();
  const maxRotation = Math.PI;
  const targetRotation = initialRotationY + normalizedDeltaX * maxRotation;

  pivot.rotation.y = targetRotation;
}

/** Calculates the camera's movement direction based on pressed keyboard's keys and camera rotation. */
function calculateDirection(keyStates: { [key: string]: boolean }, pivot: Babylon.TransformNode) {
  const forwardVector = new Vector3(
    Math.sin(pivot.rotation.y),
    0,
    Math.cos(pivot.rotation.y)
  );

  const rightVector = new Vector3(
    Math.sin(pivot.rotation.y - Math.PI / 2),
    0,
    Math.cos(pivot.rotation.y - Math.PI / 2)
  );

  let direction = Vector3.Zero();

  if (keyStates["KeyW"]) {
    direction.addInPlace(forwardVector);
  }

  if (keyStates["KeyS"]) {
    direction.subtractInPlace(forwardVector);
  }

  if (keyStates["KeyA"]) {
    direction.addInPlace(rightVector);
  }

  if (keyStates["KeyD"]) {
    direction.subtractInPlace(rightVector);
  }

  direction.normalize();
  return direction;
}

/** Adjusts the camera zoom based on an arc movement. */
function adjustCameraZoomByArcMovement(camera: FreeCamera, currentZoom: number) {
  const maxPosZ = 0;
  const minPosZ = -5;
  const maxPosY = 30;
  const minPosY = 5;

  const pos = circularArcInterpolation(minPosY, minPosZ, maxPosY, maxPosZ, currentZoom);

  camera.position.z = pos.z;
  camera.position.y = pos.y;
}

/** Adjusts the camera's rotation to keep it looking at the scene center. */
function targetSceneCenter(camera: FreeCamera) {
  const maxRotationX = Math.PI / 2;
  const minRotationX = 0.61;

  const rotation = camera.rotation.clone();
  camera.setTarget(new Vector3(0, 0, 0));
  rotation.x =  clamp(camera.rotation.x, minRotationX, maxRotationX);
  camera.rotation = rotation;
}

/** Rotates the camera when the mouse cursor is in the left or right edge of the screen */
function edgeBasedMouseRotationControl(mouseX: number, pivot: TransformNode, deltaTime: number) {
  const activatorSize = 25;
  const speed = Store.settings.edgeScrollingRotationSpeed;

  if (mouseX <= activatorSize) {
    pivot.rotation.y += speed * deltaTime;
  }
  else if (document.body.clientWidth - mouseX <= activatorSize) {
    pivot.rotation.y -= speed * deltaTime;
  }
}

/** Zooms the camera when the mouse cursor is in the top or bottom edge of the screen */
function edgeBasedMouseZoomControl(mouseY: number, deltaTime: number, zoomVelocity: number, minZoomVelocity: number, maxZoomVelocity: number) {
  const activatorSize = 25;
  let delta = 0;

  if (mouseY <= activatorSize) {
    delta = 1;
  }
  else if (document.body.clientHeight - mouseY <= activatorSize) {
    delta = -1; 
  }
  else {
    return zoomVelocity;
  }

  const zoomDelta = delta * Store.settings.edgeScrollingZoomSpeed * deltaTime;
  zoomVelocity += zoomDelta;

  return clamp(zoomVelocity, minZoomVelocity, maxZoomVelocity);
}