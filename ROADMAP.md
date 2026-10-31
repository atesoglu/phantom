# Phantom Roadmap — Realistic Fedora/Wayland-first scope

## Phase 0 — Foundation (done skeleton)
- [x] Cargo workspace + crates
- [x] Meson build skeleton
- [x] libadwaita app shell
- [x] Unix socket IPC daemon
- [x] TOML macro store
- [x] Basic keystroke injection model
- [x] Sample macro + systemd unit + README

## Phase 1 — Core engine (highly realistic)
- [ ] Expand Macro/Action schema (delay, mouse basic, clipboard, window focus, shell)
- [ ] Playback scheduler with pause/resume/cancel and repeat counts
- [ ] Macro comments in TOML schema
- [ ] Daemon action executor trait and backend injection
- [ ] Hotkey trigger matching (hotkey press)
- [ ] Cron/systemd scheduling integration UI
- [ ] Active-window trigger matching (title/process)
- [ ] Text expansion capture + injection
- [ ] Profiles (config dirs + UI switch)
- [ ] CLI tool (list/run/validate/status) over IPC
- [ ] `phantom --doctor` diagnostics

## Phase 2 — Native polish (high/medium realistic)
- [ ] Global shortcut integration via compositor/portal
- [ ] Wayland portal screen capture (optional)
- [ ] Clipboard read/write support
- [ ] Undo/redo in macro editor
- [ ] Action multi-select and reorder UX
- [ ] Variables + basic if/repeat logic
- [ ] Basic window focus helpers
- [ ] Tray/minimize support where available
- [ ] Packaging (Flatpak + RPM + AppImage)

## Phase 3 — Advanced (medium realistic)
- [ ] Macro recording engine (input capture behind permissions)
- [ ] Image/color screen waits (optional module)
- [ ] Shell action retries/timeouts/backoff
- [ ] Profile switch on trigger
- [ ] Local MCP server (trusted agents only)
- [ ] More compositor-specific backends (KWin/Sway/Mutter)

## Explicitly deferred / platform-limited
- Full cross-platform Windows parity
- Pixel-perfect image search across monitor gaps
- Display-less server mode (automation needs desktop session)
- Heavy plugin ecosystem