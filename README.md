# Phantom

Minimal desktop macro/automation app. Fedora 44 / Wayland first.

## Build (Fedora 44)

Install toolchain:
```
sudo dnf install rust cargo meson ninja-build gcc \
  gtk4-devel libadwaita-devel
```

Build:
```
meson setup build
meson compile -C build
```

Or directly with Cargo:
```
cargo build --workspace
cargo test --workspace
```

Run daemon:
```
./target/debug/phantom-daemon
# or via systemd user:
systemctl --user enable --now phantom.service
```

Run UI:
```
./target/debug/phantom-ui
```

## Storage
- Config: ~/.config/tinymacro/*.toml
- Runtime socket: $XDG_RUNTIME_DIR/phantom.sock

## Tests
```
cargo test --workspace
```