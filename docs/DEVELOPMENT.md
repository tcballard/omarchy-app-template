# Build and run

Requires Rust 1.85.0 (rustup), a C++17 compiler, binutils, pkg-config and Qt 6 Widgets development files. On Arch/Omarchy these are provided by the normal rustup/base-devel/pkgconf/qt6-base packages; install dependencies using your normal package management workflow. The template does not install system dependencies itself.

```bash
cargo test --locked --no-default-features
cargo build --locked --release
./target/release/omarchy-app-starter
```

The default GUI feature compiles the small Qt bridge; the no-default-features test command exercises only the pure Rust core. Cross-compilation is deliberately refused until a target Qt toolchain is configured.

For a headless startup and callback smoke test:

```bash
QT_QPA_PLATFORM=offscreen timeout 15s ./target/release/omarchy-app-starter --smoke-test
```

Desktop integration files are under packaging, with matching application ID io.github.tcballard.app_starter. When building a real Arch package, install the binary into /usr/bin, the desktop entry into /usr/share/applications and the SVG into /usr/share/icons/hicolor/scalable/apps, all under pkgdir. Do not copy directly into live /usr during builds.

Development runs do not install files or create application data. Remove the checkout to remove the starter. A later package should use pacman removal and leave user data intact; document that path when the package exists.

Live acceptance remains: Wayland identity, keyboard and mouse behaviour, focus/close, 100%/150%/200% scaling, theme appearance and multi-monitor movement. The bundled icon is original placeholder artwork, not a final app icon.
