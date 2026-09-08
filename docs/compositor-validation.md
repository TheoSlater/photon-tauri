# Compositor validation

Run Photon on a real desktop session. Do not count startup logs as visual validation.

## Linux

1. Launch with `bunx tauri dev`.
2. Confirm page fills client area and opaque overlay sits above page content.
3. Click button, link, input, and textarea outside overlay.
4. Scroll, hover, select text, type, maximize, restore, minimize, and resize rapidly.
5. Confirm overlay stays fixed, page bounds follow client size, and no stale frames remain.
6. Close window. Confirm `photon: webviews destroyed` appears.
7. Repeat under Wayland and X11 when available.

## Windows

1. Launch Photon with WebView2 installed.
2. Confirm `backend = webview2, render_mode = composited`.
3. Scroll, click, type, resize, maximize/restore, and move between monitors.
4. Verify cursor changes, overlay stacking, no ghosting, and clean shutdown.

## macOS

1. Launch Photon on macOS.
2. Confirm `backend = wkwebview, render_mode = composited`.
3. Repeat page input, resize, maximize/restore, overlay, and shutdown checks.
