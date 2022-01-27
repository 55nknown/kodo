// >/<

use std::sync::Arc;

use eframe::egui::CentralPanel;
use eframe::egui::*;
use eframe::epi;

use crate::ui::about::AboutPage;
use crate::ui::explorer::Explorer;

#[derive(Default)]
pub struct KodoApp {
    explorer: Explorer,
    dropped_files: Vec<DroppedFile>,
}

impl KodoApp {
    // drag and drop testing
    fn detect_files_being_dropped(&mut self, ctx: &Context) {
        // Preview hovering files:
        if !ctx.input().raw.hovered_files.is_empty() {
            let mut text = "Dropping files:\n".to_owned();
            for file in &ctx.input().raw.hovered_files {
                if let Some(path) = &file.path {
                    text += &format!("\n{}", path.display());
                } else if !file.mime.is_empty() {
                    text += &format!("\n{}", file.mime);
                } else {
                    text += "\n???";
                }
            }

            let painter =
                ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

            let screen_rect = ctx.input().screen_rect();
            painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
            painter.text(
                screen_rect.center(),
                Align2::CENTER_CENTER,
                text,
                FontId::new(24.0, FontFamily::Monospace),
                Color32::WHITE,
            );
        }

        // Collect dropped files:
        if !ctx.input().raw.dropped_files.is_empty() {
            self.dropped_files = ctx.input().raw.dropped_files.clone();
        }
    }
}

impl epi::App for KodoApp {
    fn name(&self) -> &str {
        "Kodo" // + open folder name
    }

    fn clear_color(&self) -> Rgba {
        Color32::TRANSPARENT.into()
    }

    /// Called once before the first frame.
    fn setup(&mut self, _ctx: &Context, frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        frame.set_window_size(Vec2::new(400.0, 600.0));
    }

    fn update(&mut self, ctx: &Context, _frame: &epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            AboutPage::default().ui(ui);
        });

        self.detect_files_being_dropped(ctx);
    }
}

// todo: min window size
