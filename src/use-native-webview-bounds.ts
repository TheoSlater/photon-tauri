import { invoke } from "@tauri-apps/api/core";
import { useEffect, useRef } from "react";

export type NativeWebViewBounds = {
  x: number;
  y: number;
  width: number;
  height: number;
};

function rounded(value: number, minimum = -Infinity) {
  return Number.isFinite(value) ? Math.max(minimum, Math.round(value)) : 0;
}

function sameBounds(a: NativeWebViewBounds, b: NativeWebViewBounds) {
  return a.x === b.x && a.y === b.y && a.width === b.width && a.height === b.height;
}

export function useNativeWebViewBounds() {
  const frameRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const frame = frameRef.current;
    if (!frame) return;

    let animationFrame: number | undefined;
    let lastSent: NativeWebViewBounds | undefined;

    const sendBounds = () => {
      animationFrame = undefined;
      const rect = frame.getBoundingClientRect();
      const bounds: NativeWebViewBounds = {
        x: rounded(rect.left),
        y: rounded(rect.top),
        width: rounded(rect.width, 0),
        height: rounded(rect.height, 0),
      };

      if (lastSent && sameBounds(lastSent, bounds)) return;
      lastSent = bounds;
      void invoke("browser_set_viewport", { bounds }).catch((error: unknown) => {
        console.error("photon: failed to update native webview bounds", error);
      });
    };

    const schedule = () => {
      if (animationFrame === undefined) animationFrame = requestAnimationFrame(sendBounds);
    };
    const resizeObserver =
      typeof ResizeObserver === "undefined" ? undefined : new ResizeObserver(schedule);
    resizeObserver?.observe(frame);
    if (frame.parentElement) resizeObserver?.observe(frame.parentElement);
    const mutationObserver =
      typeof MutationObserver === "undefined" ? undefined : new MutationObserver(schedule);
    mutationObserver?.observe(document.documentElement, {
      attributes: true,
      childList: true,
      subtree: true,
    });
    window.addEventListener("resize", schedule);
    schedule();

    return () => {
      resizeObserver?.disconnect();
      mutationObserver?.disconnect();
      window.removeEventListener("resize", schedule);
      if (animationFrame !== undefined) cancelAnimationFrame(animationFrame);
    };
  }, []);

  return frameRef;
}
