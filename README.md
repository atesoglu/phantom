# Phantom

Minimal desktop macro/automation app. Fedora 44 / Wayland first. Single-process app: automation runs while the app is open.

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

Run UI:
```
./target/debug/phantom-ui
```

## Storage
- Config: ~/.config/macros/*.toml

## Notes
- /dev/uinput access may require `input` group membership.
- No background daemon/service in this version.