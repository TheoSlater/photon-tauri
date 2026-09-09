import { Button } from "@heroui/react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, Square, X } from "lucide-react";

const currentWindow = getCurrentWindow();

export function PhotonTitlebar() {
  return (
    <header className="photon-titlebar" aria-label="Photon">
      <div className="photon-titlebar-drag-region" data-tauri-drag-region>
        Photon
      </div>
      <div className="photon-window-controls">
        <Button
          className="photon-window-control"
          isIconOnly
          size="sm"
          variant="ghost"
          aria-label="Minimize window"
          onPress={() => void currentWindow.minimize().catch(console.error)}
        >
          <Minus aria-hidden size={14} strokeWidth={1.5} />
        </Button>
        <Button
          className="photon-window-control"
          isIconOnly
          size="sm"
          variant="ghost"
          aria-label="Maximize or restore window"
          onPress={() => void currentWindow.toggleMaximize().catch(console.error)}
        >
          <Square aria-hidden size={13} strokeWidth={1.5} />
        </Button>
        <Button
          className="photon-window-control"
          isIconOnly
          size="sm"
          variant="ghost"
          aria-label="Close window"
          onPress={() => void currentWindow.close().catch(console.error)}
        >
          <X aria-hidden size={14} strokeWidth={1.5} />
        </Button>
      </div>
    </header>
  );
}
