import { Babylon, BabylonWorld } from "../babylon/babylon";
import { clamp, lerp, timeToPercent } from "../math/utils";

type Animation = { group: Babylon.AnimationGroup, weight: number };

/** Utility object for controlling and blending animations. */
export type AnimationManager = {
  /**
   * Selects the specified animation to play.
   * 
   * ## Arguments
   * 
   * * `animation` - The name of the animation to select.
   * * `loop` - Optional. Specifies whether the animation should loop. Default is false.
   */
  select: (animation: string, loop?: boolean) => void;

  /** Returns the name of the played animation. */
  getSelected: () => string;

  /** Adds a callback function to be executed when the currently selected animation ends. */
  onEnd: (callback: () => void) => void;

  /** Unregisters all event listeners related to the AnimationManager */
  dispose: () => void;
};

/**
 * Creates an AnimationManager instance.
 * 
 * ## Arguments
 * 
 * * `world` - The BabylonWorld object.
 * * `allAnimations` - An object containing all available animations, with keys representing animation names.
 * * `blendingTime` - Optional. The duration in milliseconds for blending between animations. Default is 1000.
 */
export function createAnimationManager (world: BabylonWorld, allAnimations: { [key: string]: Babylon.AnimationGroup }, blendingTime = 1000): AnimationManager {
  const animations: { [key: string]: Animation } = {};

  for (const key of Object.keys(allAnimations)) {
    animations[key] = {
      group: allAnimations[key],
      weight: 0,
    };

    allAnimations[key].setWeightForAllAnimatables(0);
    allAnimations[key].stop();
  }
  
  let selected: string = "";
  let disableBlends: ({ animation: Animation, from: number, startTime: number })[] = [];
  let enableBlends: { animation: Animation, from: number, startTime: number } = undefined;

  /** Blends animations based on their weights and blending time. */
  function blending() {
    const time = Date.now();

    for (let i = 0; i < disableBlends.length; i++) {
      const blend = disableBlends[i];
      const weight = clamp(lerp(blend.from, 0, timeToPercent(time, blend.startTime, blend.startTime + blendingTime)), 0, 1);

      blend.animation.weight = weight;
      blend.animation.group.setWeightForAllAnimatables(weight);

      if (weight <= 0.001) {
        disableBlends.splice(i, 1);
        blend.animation.group.stop();
        i--;
      }
    }

    if (enableBlends) {
      const weight = lerp(enableBlends.from, 1, timeToPercent(time, enableBlends.startTime, enableBlends.startTime + blendingTime));
      enableBlends.animation.weight = weight;
      enableBlends.animation.group.setWeightForAllAnimatables(weight);

      if (weight > 0.999) {
        enableBlends = undefined;
      }
    }
  }

  world.scene.registerBeforeRender(blending);

  return {
    select: (animation: string, loop = false) => {
      const old = animations[selected];
      const current = animations[animation];
      selected = animation;

      if (!old) {
        current.weight = 1;
        current.group.setWeightForAllAnimatables(1);
        current.group.play(loop);
        return;
      }

      
      disableBlends.push({
        animation: old,
        from: old.weight,
        startTime: Date.now(),
      });
      
      enableBlends = {
        animation: current,
        from: current.weight,
        startTime: Date.now(),
      };

      old.group.onAnimationGroupEndObservable.clear();
      current.group.play(loop);
    },

    getSelected: () => {
      return selected;
    },

    onEnd(callback) {
      const current = animations[selected];

      if (current) {
        current.group.onAnimationGroupEndObservable.add(callback);
      }
    },

    dispose: () => {
      world.scene.unregisterBeforeRender(blending);
    }
  }
}