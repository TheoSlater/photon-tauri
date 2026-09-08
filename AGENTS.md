# AGENTS.md

## Photon Browser

Photon is a minimal, high-performance desktop browser built with:

* Tauri 2
* Rust
* React + TypeScript
* HeroUI
* Tailwind CSS
* Normal CSS
* Custom Wry fork
* Native system webviews

Supported platforms:

* Windows — WebView2
* macOS — WKWebView
* Linux — WebKitGTK

---

## Core Rules

1. **Keep everything simple.**
2. **Do not overengineer.**
3. **Do not add features that were not requested.**
4. **Do not redesign existing UI unless explicitly asked.**
5. **Do not replace working architecture without a clear technical reason.**
6. **Prefer small, focused changes over large rewrites.**
7. **Fix root causes. Do not add hacks or workarounds that hide problems.**
8. **Do not leave dead code, temporary code, commented-out experiments, or unused dependencies.**
9. **Do not create abstractions until they are actually needed.**
10. **Preserve cross-platform support.**

---

## Architecture

The browser architecture is:

```text
Tauri
└── BrowserWindow
    └── TabManager
        └── Tab
            └── BrowserPage
                └── composited Wry WebView
```

Rust owns browser state.

React renders browser UI.

Do not move browser ownership into React or Zustand.

### Browser State

Rust is authoritative for:

* tabs
* active tab
* pages
* navigation
* native webviews
* page lifecycle

React may mirror this state for rendering.

### UI State

Frontend state may own things such as:

* open menus
* popovers
* temporary input state
* UI preferences
* animation state

Do not duplicate authoritative browser state unnecessarily.

---

## Webviews

Photon uses the custom Wry fork.

Do not replace it with:

* iframes
* Electron
* CEF
* embedded Chromium
* fake webpage rendering

Page webviews use:

```rust
WebViewRenderMode::Composited
```

Native engines:

```text
Windows → WebView2
macOS   → WKWebView
Linux   → WebKitGTK
```

Keep platform-specific implementation inside `platform/`.

Do not scatter platform `cfg` logic throughout browser-domain code.

---

## Frontend

Use:

* React
* TypeScript
* HeroUI
* Tailwind
* normal CSS

### Styling

Use Tailwind for simple layout and spacing.

Example:

```tsx
className="flex items-center gap-2"
```

Use normal CSS for custom Photon styling, complex states, browser chrome, native layout synchronization, and animations.

Do not force everything into Tailwind.

Use existing HeroUI theme variables whenever possible.

Do not introduce another UI framework without explicit approval.

---

## UI Design

Photon should remain:

* minimal
* clean
* compact
* fast
* unobtrusive

Do not add decorative UI without being asked.

Avoid:

* excessive gradients
* excessive shadows
* excessive blur
* oversized controls
* unnecessary borders
* unnecessary text
* excessive rounded containers

Do not redesign something simply because you prefer another style.

---

## Performance

Performance is a core requirement.

Avoid:

* unnecessary React renders
* unnecessary native webview recreation
* polling when events are available
* expensive work on the UI thread
* unnecessary allocations in hot paths
* unnecessary background work

Inactive tabs should not be destroyed merely because they are inactive.

Do not optimize blindly. Measure when performance is uncertain.

---

## Files

Keep files focused.

Prefer files below approximately **300 lines**.

If a file becomes large because it owns multiple unrelated responsibilities, split it.

Do not split files merely to satisfy a line count.

Avoid:

* god objects
* giant components
* giant managers
* generic `utils` dumping grounds

Each module should have a clear responsibility.

---

## Dependencies

Do not add a dependency unless necessary.

Before adding one:

1. Check whether the project already provides the functionality.
2. Prefer platform/framework capabilities where appropriate.
3. Explain any significant new dependency.

Never replace existing dependencies casually.

---

## Error Handling

Do not silently ignore errors.

Do not use `unwrap()` or `expect()` in production paths unless failure is genuinely impossible and clearly justified.

Return or propagate meaningful errors.

User-facing failures should eventually be representable by the UI.

---

## Testing

After meaningful changes, run the relevant checks.

At minimum:

```bash
bun run build
cargo fmt -- --check
cargo check
cargo test
git diff --check
```

Do not claim runtime behavior was tested unless it was actually run and observed.

Do not claim Windows or macOS behavior was validated from Linux.

---

## Changes

Before changing existing architecture:

1. Understand why it exists.
2. Trace its callers and ownership.
3. Preserve existing behavior unless the task explicitly changes it.

When fixing bugs:

1. Identify the root cause.
2. Fix the root cause.
3. Remove obsolete workaround code.
4. Verify the affected behavior.

Do not perform unrelated refactors during focused tasks.

---

## Never Do Without Explicit Request

Do not independently add:

* AI features
* bookmarks
* history
* downloads UI
* profiles
* extensions
* workspaces
* tab groups
* split view
* session restore
* telemetry
* analytics
* cloud services
* accounts
* onboarding
* additional UI libraries

Implement only the requested scope.

---

## Final Rule

**Make the smallest clean change that fully solves the requested problem.**

If existing code already solves part of the problem, reuse it.

Do not turn a simple task into a framework.
