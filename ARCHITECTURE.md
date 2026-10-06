# Architecture

Rust owns application state in src/lib.rs. The binary passes one state pointer and a callback into a small Qt Widgets bridge in ui/window.cpp. The callback is invoked synchronously on the UI thread; Qt retains neither pointer after its event loop returns. C++ exceptions are caught before crossing the C ABI. The saturating state operation does not panic.

Qt parent ownership releases widgets before QApplication is destroyed. Rust state and C strings remain alive for the complete event loop. There are no worker threads, network access or persistent settings yet.

The application ID matches the Wayland desktop identity, desktop entry basename and icon basename. Qt follows the desktop's configured Qt platform theme; automatic live Omarchy theme reload is not promised. The starter does not depend on shell-only qs.Commons imports.

The Qt bridge has no Rust crate dependencies. Cargo.lock locks the local package; rust-toolchain.toml records the compiler. Qt is dynamically linked from the build host. Native CI compiles the bridge, runs pure state tests and exercises an offscreen button click. On-device focus, scaling and theme behaviour still require live acceptance.
