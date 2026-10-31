# Phantom Roadmap — Single-process, Fedora/Wayland-first

## Phase 0 — Foundation
- [x] Cargo workspace + crates
- [x] Meson build skeleton
- [x] libadwaita app shell
- [x] TOML macro store
- [x] Debug mouse record/replay via uinput
- [x] README/build instructions

## Phase 1 — Core engine
- [ ] Macro list UI
- [ ] Macro editor UI
- [ ] Enable/disable toggles
- [ ] Repeat/delay/pause/cancel
- [ ] Hotkey triggers while app open
- [ ] Active-window triggers
- [ ] Basic text expansion
- [ ] Profiles

## Phase 2 — Polish
- [ ] CLI helper
- [ ] Undo/redo editor
- [ ] Variables/basic logic
- [ ] Packaging (Flatpak/RPM/AppImage)
- [ ] Diagnostics command

## Explicitly deferred
- Background daemon/systemd service (removed by design)
- MCP server
- Advanced image/color waits