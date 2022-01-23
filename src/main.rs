#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = kodo_editor::KodoApp::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.transparent = true;
    eframe::run_native(Box::new(app), native_options);
}
