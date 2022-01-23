use eframe::egui::epaint::Shadow;
use eframe::egui::{self, Color32, Stroke, TextEdit, TextStyle, Ui};
use eframe::egui::{ScrollArea, Vec2};
use eframe::epi;
use native_dialog::FileDialog;

use crate::{theme, tree};

pub struct KodoApp {
    buffer: String,
}

impl Default for KodoApp {
    fn default() -> Self {
        Self {
            buffer: String::from(""),
        }
    }
}

impl KodoApp {
    fn max_text_width(&self, ui: &Ui) -> f32 {
        let text_style = TextStyle::Monospace;
        let col_width = ui.fonts()[text_style].glyph_width('0');
        self.buffer
            .split("\n")
            .map(|line| col_width as usize * line.len())
            .max()
            .unwrap() as f32
            + col_width * 4_f32 // clearance
    }
}

impl epi::App for KodoApp {
    fn name(&self) -> &str {
        "Kodo"
    }

    fn clear_color(&self) -> egui::Rgba {
        egui::Color32::TRANSPARENT.into()
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        ctx.set_visuals(theme::dark());
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let top_frame = egui::Frame {
            margin: Vec2::ZERO,
            corner_radius: 0.0,
            shadow: Shadow {
                extrusion: 0.0,
                color: Color32::TRANSPARENT,
            },
            fill: theme::Color(theme::BACKGROUND),
            stroke: Stroke::none(),
        };

        // egui::TopBottomPanel::top("top_panel")
        //     .frame(top_frame)
        //     .show(ctx, |ui| {
        //         egui::menu::bar(ui, |ui| {
        //             ui.spacing_mut().button_padding = Vec2::new(12.0, 6.0);
        //             ui.menu_button("File", |ui| {
        //                 ui.spacing_mut().button_padding = Vec2::new(12.0, 6.0);
        //                 if ui.button("Open File...").clicked() {
        //                     if let Ok(path) = FileDialog::new().show_open_single_file() {
        //                         if let Some(path) = path {
        //                             println!("{:?}", path);
        //                         }
        //                     }
        //                     ui.close_menu();
        //                 } else if ui.button("Open Folder...").clicked() {
        //                     if let Ok(path) = FileDialog::new().show_open_single_dir() {
        //                         if let Some(path) = path {
        //                             println!("{:?}", path);
        //                         }
        //                     }
        //                     ui.close_menu();
        //                 } else if ui.button("Exit").clicked() {
        //                     frame.quit();
        //                 }
        //             });
        //         });
        //     });

        let side_frame = egui::Frame {
            margin: Vec2::ZERO,
            corner_radius: 0.0,
            shadow: Shadow {
                extrusion: 0.0,
                color: Color32::TRANSPARENT,
            },
            fill: theme::Color(theme::BACKGROUND),
            stroke: Stroke::none(),
        };

        egui::SidePanel::left("explorer")
            .frame(side_frame)
            .resizable(true)
            .min_width(100.0)
            .max_width(500.0)
            .show(ctx, |ui| {
                tree::Tree::demo().ui(ui);
            });

        let main_frame = egui::Frame {
            margin: Vec2::ZERO,
            corner_radius: 0.0,
            shadow: Shadow {
                extrusion: 0.0,
                color: Color32::TRANSPARENT,
            },
            fill: theme::Color(theme::BACKGROUND & 0xBBFFFFFF),
            stroke: Stroke::none(),
        };
        egui::CentralPanel::default()
            .frame(main_frame)
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = Vec2::new(4.0, 4.0);

                ScrollArea::both()
                    .always_show_scroll(true)
                    .stick_to_bottom()
                    .show_viewport(ui, |ui, _viewport| {
                        ui.set_min_width(self.max_text_width(ui));
                        ui.style_mut().visuals.widgets.hovered.bg_stroke =
                            Stroke::new(0.0, Color32::RED);
                        ui.style_mut().visuals.widgets.active.bg_stroke =
                            Stroke::new(0.0, Color32::RED);
                        ui.style_mut().visuals.widgets.inactive.bg_stroke =
                            Stroke::new(0.0, Color32::RED);
                        ui.style_mut().visuals.widgets.active.corner_radius = 0.0;

                        let theme = crate::syntax_highlighting::CodeTheme::from_memory(ui.ctx());

                        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                            let mut layout_job = crate::syntax_highlighting::highlight(
                                ui.ctx(),
                                &theme,
                                string,
                                "rs",
                            );
                            layout_job.wrap_width = wrap_width;
                            ui.fonts().layout_job(layout_job)
                        };

                        ui.add(
                            TextEdit::multiline(&mut self.buffer)
                                .text_style(egui::TextStyle::Monospace) // for cursor height
                                .code_editor()
                                .desired_width(f32::INFINITY)
                                .lock_focus(true)
                                .layouter(&mut layouter),
                        );
                    });
            });
    }
}
