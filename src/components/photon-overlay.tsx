import type { CSSProperties, PropsWithChildren } from "react";
import {
  createContext,
  createElement,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import { invoke } from "@tauri-apps/api/core";
import { createPortal } from "react-dom";

type OverlayBounds = {
  x: number;
  y: number;
  width: number;
  height: number;
};

export type OverlayInteractionMode = "passthrough" | "dismiss" | "modal";

type OverlayEntry = {
  id: string;
  element: HTMLDivElement;
  interaction: OverlayInteractionMode;
  order: number;
  onDismiss?: () => void;
  dismissOnOutsidePress: boolean;
};

type OverlayContextValue = {
  root: HTMLDivElement | null;
  nextOrder: () => number;
  register: (entry: OverlayEntry) => void;
  unregister: (id: string) => void;
};

const OverlayContext = createContext<OverlayContextValue | null>(null);

function rounded(value: number, minimum = -Infinity) {
  return Number.isFinite(value) ? Math.max(minimum, Math.round(value)) : 0;
}

function sameBounds(a: OverlayBounds | undefined, b: OverlayBounds) {
  return !!a && a.x === b.x && a.y === b.y && a.width === b.width && a.height === b.height;
}

export function PhotonOverlayRoot({ children }: PropsWithChildren) {
  const [root, setRoot] = useState<HTMLDivElement | null>(null);
  const order = useRef(0);
  const entries = useRef(new Map<string, OverlayEntry>());
  const context = useMemo(
    () => ({
      root,
      nextOrder: () => order.current++,
      register: (entry: OverlayEntry) => entries.current.set(entry.id, entry),
      unregister: (id: string) => entries.current.delete(id),
    }),
    [root],
  );

  useEffect(() => {
    if (!root) return;

    const outsidePress = (event: PointerEvent) => {
      const target = event.target;
      const active = [...entries.current.entries()]
        .filter(([, entry]) => !entry.element.contains(target as Node))
        .map(([id, entry]) => ({ id, entry }));
      const modal = active
        .filter(({ entry }) => entry.interaction === "modal")
        .sort((a, b) => b.entry.order - a.entry.order)[0];
      const dismiss = active
        .filter(({ entry }) => entry.interaction === "dismiss")
        .sort((a, b) => b.entry.order - a.entry.order)[0];
      const policy = modal ?? dismiss;
      if (!policy) return;

      event.preventDefault();
      event.stopImmediatePropagation();
      if (policy.entry.interaction === "dismiss" || policy.entry.dismissOnOutsidePress) {
        policy.entry.onDismiss?.();
      }
    };

    const modalWheel = (event: WheelEvent) => {
      const target = event.target;
      const modal = [...entries.current.values()]
        .filter(
          (entry) =>
            entry.interaction === "modal" && !entry.element.contains(target as Node),
        )
        .sort((a, b) => b.order - a.order)[0];
      if (modal) {
        event.preventDefault();
        event.stopImmediatePropagation();
      }
    };

    document.addEventListener("pointerdown", outsidePress, true);
    document.addEventListener("wheel", modalWheel, true);
    return () => {
      document.removeEventListener("pointerdown", outsidePress, true);
      document.removeEventListener("wheel", modalWheel, true);
    };
  }, [root]);

  return createElement(
    OverlayContext.Provider,
    { value: context },
    createElement("div", { ref: setRoot, className: "photon-overlay-root" }),
    root ? createPortal(children, root) : null,
  );
}

export type PhotonOverlayProps = PropsWithChildren<{
  id: string;
  interaction?: OverlayInteractionMode;
  order?: number;
  onDismiss?: () => void;
  dismissOnOutsidePress?: boolean;
  className?: string;
  style?: CSSProperties;
}>;

export function PhotonOverlay({
  id,
  interaction = "passthrough",
  order,
  onDismiss,
  dismissOnOutsidePress = false,
  className,
  style,
  children,
}: PhotonOverlayProps) {
  const context = useContext(OverlayContext);
  const elementRef = useRef<HTMLDivElement>(null);
  const orderRef = useRef<number | undefined>(order);
  if (order !== undefined) orderRef.current = order;
  else if (orderRef.current === undefined && context) orderRef.current = context.nextOrder();

  useEffect(() => {
    const element = elementRef.current;
    if (!context?.root || !element) return;

    let animationFrame: number | undefined;
    let lastSent: OverlayBounds | undefined;
    context.register({
      id,
      element,
      interaction,
      order: orderRef.current ?? 0,
      onDismiss,
      dismissOnOutsidePress,
    });

    const sendBounds = () => {
      animationFrame = undefined;
      const rect = element.getBoundingClientRect();
      const bounds: OverlayBounds = {
        x: rounded(rect.left),
        y: rounded(rect.top),
        width: rounded(rect.width, 0),
        height: rounded(rect.height, 0),
      };
      if (sameBounds(lastSent, bounds)) return;
      lastSent = bounds;
      void invoke("browser_register_overlay", {
        overlay: {
          id,
          bounds,
          order: orderRef.current ?? 0,
          interaction,
          dismissOnOutsidePress,
        },
      }).catch((error: unknown) => console.error("photon: failed to register overlay", error));
    };

    const schedule = () => {
      if (animationFrame === undefined) animationFrame = requestAnimationFrame(sendBounds);
    };
    const resizeObserver =
      typeof ResizeObserver === "undefined" ? undefined : new ResizeObserver(schedule);
    resizeObserver?.observe(element);
    const mutationObserver =
      typeof MutationObserver === "undefined" ? undefined : new MutationObserver(schedule);
    mutationObserver?.observe(document.documentElement, {
      attributes: true,
      childList: true,
      subtree: true,
    });
    window.addEventListener("resize", schedule);
    window.addEventListener("scroll", schedule, true);
    schedule();

    return () => {
      resizeObserver?.disconnect();
      mutationObserver?.disconnect();
      window.removeEventListener("resize", schedule);
      window.removeEventListener("scroll", schedule, true);
      if (animationFrame !== undefined) cancelAnimationFrame(animationFrame);
      context.unregister(id);
      void invoke("browser_unregister_overlay", { id }).catch((error: unknown) =>
        console.error("photon: failed to unregister overlay", error),
      );
    };
  }, [context, id, interaction, order, onDismiss, dismissOnOutsidePress]);

  if (!context?.root) return null;
  return createPortal(
    createElement(
      "div",
      {
        ref: elementRef,
        className: ["photon-overlay", className].filter(Boolean).join(" "),
        "data-photon-overlay": id,
        "data-interaction": interaction,
        style,
      },
      children,
    ),
    context.root,
  );
}
