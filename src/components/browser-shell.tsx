import type { PropsWithChildren } from "react";
import { useNativeWebViewBounds } from "../use-native-webview-bounds";
import { PhotonTitlebar } from "./photon-titlebar";

export function WebViewFrame() {
  const frameRef = useNativeWebViewBounds();
  return <div ref={frameRef} className="photon-page-frame" aria-hidden="true" />;
}

export function BrowserViewport({ children }: PropsWithChildren) {
  return <div className="photon-page-area">{children}</div>;
}

export function BrowserShell() {
  return (
    <main className="photon-shell">
      <PhotonTitlebar />
      <BrowserViewport>
        <WebViewFrame />
      </BrowserViewport>
    </main>
  );
}
