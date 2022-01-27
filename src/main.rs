#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use std::path::Path;

fn load_icon(path: &Path) -> Option<eframe::epi::IconData> {
    let (icon_rgba, icon_width, icon_height) = {
        let image = match image::open(path) {
            Ok(img) => img.into_rgba8(),
            Err(err) => {
                eprintln!("{err}");
                return None;
            }
        };

        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Some(eframe::epi::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    })
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = kodo_editor::KodoApp::default();
    let native_options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: false,
        drag_and_drop_support: true,
        icon_data: load_icon(Path::new("assets/kodo_icon_circle.png")),
        initial_window_size: None,
        resizable: true,
        transparent: false,
    };
    eframe::run_native(Box::new(app), native_options);
}
