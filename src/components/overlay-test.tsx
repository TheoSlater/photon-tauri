import { useState } from "react";
import { PhotonOverlay } from "./photon-overlay";

const modes = ["passthrough", "dismiss", "modal"] as const;

export function OverlayTest() {
  const [clicked, setClicked] = useState(false);
  const [interaction, setInteraction] = useState<(typeof modes)[number]>("passthrough");
  const [open, setOpen] = useState(true);
  if (import.meta.env.VITE_PHOTON_OVERLAY_TEST !== "1") return null;

  return (
    <>
      <PhotonOverlay
        id="overlay-test-lower"
        interaction="passthrough"
        order={10}
        className="photon-overlay-test"
      >
        <div className="photon-overlay-test-panel">
          <span>Lower overlay</span>
          <button type="button" onClick={() => setOpen(true)}>
            Open
          </button>
        </div>
      </PhotonOverlay>
      {open && (
        <PhotonOverlay
          id="overlay-test-upper"
          interaction={interaction}
          order={20}
          className="photon-overlay-test-upper"
          onDismiss={() => setOpen(false)}
        >
          <div className="photon-overlay-test-panel">
            <span>Overlay: {interaction}</span>
            <button type="button" onClick={() => setClicked(true)}>
              {clicked ? "Clicked" : "Click me"}
            </button>
            <button
              type="button"
              onClick={() =>
                setInteraction(modes[(modes.indexOf(interaction) + 1) % modes.length])
              }
            >
              Next mode
            </button>
          </div>
        </PhotonOverlay>
      )}
    </>
  );
}
