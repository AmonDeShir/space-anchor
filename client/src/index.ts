import { ArcRotateCamera, Engine, HemisphericLight, Mesh, MeshBuilder, Scene, Vector3 } from "@babylonjs/core";
import { createGame, world } from "./game/game";
import { createUI } from "./ui/ui";

createGame(world);
createUI();
