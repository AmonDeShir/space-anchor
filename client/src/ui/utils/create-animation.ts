import { Setter, createEffect, createSignal, onCleanup } from "solid-js";
import { gsap } from "gsap";

export type Animation<T = HTMLElement> = {
  ref: Setter<T>;  
  start: () => void;
}

export function createAnimation<T = HTMLElement>(animation: (tl: gsap.core.Timeline, ref: T) => void, defaultValue?: T): Animation<T> {
  const [ref, setRef] = createSignal<T>(defaultValue ?? null);
  
  let tl: gsap.core.Timeline;

  onCleanup(() => {
    if (tl) {
      tl.kill();
    }
  });

  return { 
    ref: setRef,
    start: () => {
      if (!ref()) {
        return;
      }

      tl = gsap.timeline();
      animation(tl, ref());
    }
  };
}

export function setRefFor(...animatons: Animation<HTMLElement>[]) {
  return (ref: HTMLElement) => {
    for (let animation of animatons) {
      animation.ref(ref);
    }
  }
}

export function setRefForAdv<T>(setter: (ref: HTMLElement, old: T) => T, ...animatons: Animation<T>[]) {
  return (ref: HTMLElement) => {
    for (let animation of animatons) {
      animation.ref((old) => setter(ref, old));
    }
  }
}