# Compositor validation

Run Photon on a real desktop session. Do not count startup logs as visual validation.

## Linux

1. Launch with `bunx tauri dev`.
2. Confirm page fills client area and frontend sits above page content.
3. Click button, link, input, and textarea on the page.
4. Scroll, hover, select text, type, maximize, restore, minimize, and resize rapidly.
5. Confirm frontend stays fixed, page bounds follow client size, and no stale frames remain.
6. Close window. Confirm `photon: webviews destroyed` appears.
7. Repeat under Wayland and X11 when available.

### NVIDIA + Wayland compatibility

Photon automatically sets `__NV_DISABLE_EXPLICIT_SYNC=1` before GTK/WebKitGTK
initialization when it detects Wayland and the NVIDIA kernel modules. Existing
user values always win. Set `PHOTON_DISABLE_LINUX_GPU_WORKAROUNDS=1` to disable
the automatic compatibility behavior for testing.

If WebKitGTK still fails to render, use this manual last-resort fallback:

```bash
# Preferred affected-NVIDIA workaround
__NV_DISABLE_EXPLICIT_SYNC=1 photon

# Last-resort WebKit fallback; may reduce rendering performance
WEBKIT_DISABLE_DMABUF_RENDERER=1 photon
```

Photon never sets `WEBKIT_DISABLE_DMABUF_RENDERER` automatically.

## Windows

1. Launch Photon with WebView2 installed.
2. Confirm `backend = webview2, render_mode = composited`.
3. Scroll, click, type, resize, maximize/restore, and move between monitors.
4. Verify cursor changes, no ghosting, and clean shutdown.

## macOS

1. Launch Photon on macOS.
2. Confirm `backend = wkwebview, render_mode = composited`.
3. Repeat page input, resize, maximize/restore, and shutdown checks.
