import type { CSSProperties } from "react";
import { PAGE_PADDING, TOP_BAR_HEIGHT } from "./layout";

function TopBar() {
  return (
    <div className="photon-topbar" aria-label="Photon">
      <span>Photon</span>
    </div>
  );
}

function WebViewFrame() {
  return <div className="photon-page-frame" aria-hidden="true" />;
}

function BrowserShell() {
  const layoutStyle = {
    "--photon-topbar-height": `${TOP_BAR_HEIGHT}px`,
    "--photon-page-padding": `${PAGE_PADDING}px`,
  } as CSSProperties;

  return (
    <main className="photon-shell" style={layoutStyle}>
      <TopBar />
      <WebViewFrame />
    </main>
  );
}

function App() {
  return <BrowserShell />;
}

export default App;
