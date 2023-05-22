import { onCleanup } from "solid-js";
import { gsap } from "gsap";

export function createAnimation(animation: (tl: gsap.core.Timeline) => void): () => void {
  let tl: gsap.core.Timeline;

  onCleanup(() => {
    tl!.kill();
  });

  return () => { tl = gsap.timeline(); animation(tl) };
}