import { FinalStateMachine } from "../finite-state-machine/finite-state-machine";
import { AnimationManager } from "../animations/animations";
import { BabylonWorld } from "../babylon/babylon";

/** Type definition for different states of the admiral finite state machine */
type States = { "appears": [], "disappear": [], "idle": [], "move": [], "readyToDelete": [] };

export type AdmiralFiniteStateMachine = FinalStateMachine.FiniteStateMachine<States>;

/**
 * Creates an instance of the Admiral finite state machine.
 * 
 * ## Arguments
 * 
 * * `animations` - The animation manager.
 * * `world` - The Babylon world instance.
 */
export function createAdmiralFiniteStateMachine(animations: AnimationManager, world: BabylonWorld): AdmiralFiniteStateMachine {
  const store: FinalStateMachine.Store<States> = {
    appears: Appears(animations),
    disappear: Disappears(animations),
    idle: Idle(animations),
    move: Move(animations),
    readyToDelete: { name: "readyToDelete" }
  };

  return FinalStateMachine.create(world, store, "idle");
}

type FSMState = FinalStateMachine.StateConstructor<States, AnimationManager>;

/** 
 * State definition for the "appears" state.
 * This state occurs when the admiral appears on the map.  
 */
const Appears: FSMState = (animations) => ({
  name: "appears",
  enter(_, set) {    
    animations.select("appear");
    animations.onEnd(() => set("idle"));
  }
});

/** 
 * State definition for the "disappear" state.
 * This state occurs when the admiral is removed from the map.
 */
const Disappears: FSMState = (animations) => ({
  name: "disappear",
  enter(_, set) {
    animations.select("disappear");
    animations.onEnd(() => set("readyToDelete"));
  },
});

/** 
 * State definition for the "idle" state.
 * This state is used when the admiral is standing still.
 */
const Idle: FSMState = (animations) => ({
  name: "idle",
  enter() {
    animations.select("idle", true);
  }
});

/** 
 * State definition for the "move" state.
 * This state occurs when the admiral is moving.
 */
const Move: FSMState = (animations) => ({
  name: "move",
  enter() {
    animations.select("move", true);
  },
});