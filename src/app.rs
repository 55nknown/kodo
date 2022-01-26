// >/<

use eframe::egui;
use eframe::epi;

pub struct KodoApp {}

impl Default for KodoApp {
    fn default() -> Self {
        Self {}
    }
}

impl epi::App for KodoApp {
    fn name(&self) -> &str {
        "Kodo" // + open folder name
    }

    fn clear_color(&self) -> egui::Rgba {
        egui::Color32::TRANSPARENT.into()
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    fn update(&mut self, _ctx: &egui::CtxRef, _frame: &epi::Frame) {}
}
