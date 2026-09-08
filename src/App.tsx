import { useNativeWebViewBounds } from "./use-native-webview-bounds";

function TopBar() {
  return (
    <div className="photon-topbar" aria-label="Photon">
      <span>Photon</span>
    </div>
  );
}

function WebViewFrame() {
  const frameRef = useNativeWebViewBounds();
  return <div ref={frameRef} className="photon-page-frame" aria-hidden="true" />;
}

function BrowserShell() {
  return (
    <main className="photon-shell">
      <TopBar />
      <div className="photon-page-area">
        <WebViewFrame />
      </div>
    </main>
  );
}

function App() {
  return <BrowserShell />;
}

export default App;
