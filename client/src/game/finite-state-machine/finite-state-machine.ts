import { BabylonWorld } from "../babylon/babylon";

export namespace  FinalStateMachine {
  
  export type Store<T> = { [key in keyof T]: State<T> }
  
  /** Type representing a single state in the FSM. */
  export type State<T> = {
    
    /**
     * Optional function called when the state is selected.
     * 
     * ## Arguments
     * 
     * * `prev` - The previous state.
     * * `set` - Function to change the selected state.
    */
    enter?: (prev: State<T>, set: (state: keyof T) => void) => void,
    
    /**
     * 
     * Optional function called every frame when the state is selected.
     * 
     * ## Arguments
     * 
     * * `set` - Function to change the selected state.
     */
    update?: (set: (state: keyof T) => void) => void,
    

    /** Optional function called when the state is changed to another state. */
    exit?: () => void,
    
    /** The name of the state. */
    name: keyof T,
  }
  
  export type StateConstructor<T, I> = (item: I) => State<T>;


  export type FiniteStateMachine<T> = {
    /** Changes the selected state. */
    set: (state: keyof T) => void;

    /** Gets the name of the current state. */
    get: () => keyof T,

    /** Unregisters all event listeners related to the FiniteStateMachine. */
    dispose: () => void,
  }
  
  /**
   * Creates a new finite state machine.
   * 
   * ## Arguments
   * 
   * * `world` - The BabylonWorld instance.
   * * `store` - Object containing all available states.
   * * `selected` - The initial selected state.
   */
  export function create<T>(world: BabylonWorld, store: Store<T>, selected: keyof T): FiniteStateMachine<T> {
    function setState(state: keyof T) {
      let prev = store[selected];
      let next = store[state];
  
      selected = undefined;
  
      if (prev?.exit != undefined) {
        prev.exit();
      }
  
      if (next?.enter != undefined) {
        next.enter(prev, setState);
      }
  
      selected = next.name;
    }
  
    const update = () => {  
      if (store[selected]?.update != undefined) {
        store[selected].update(setState);
      }
    };
  
    world.scene.registerBeforeRender(update)
  
    return {
      set: setState,
      get: () => store[selected]?.name,
      dispose: () => world.scene.unregisterBeforeRender(update)
    }
  }
}
