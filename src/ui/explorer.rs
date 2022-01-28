#![allow(dead_code)]
use std::path::{Path, PathBuf};

use eframe::egui::epaint::Shadow;
use eframe::egui::{self, Color32, Context, Vec2};
use eframe::egui::{ScrollArea, Stroke};

use super::tree::Tree;

#[derive(Default)]
pub struct Explorer {
    tree: Option<Tree>,
}

impl Explorer {
    pub fn show(&mut self, ctx: &Context) -> ExplorerResponse {
        let side_frame = egui::Frame {
            margin: Vec2::ZERO,
            corner_radius: 0.0,
            shadow: Shadow {
                extrusion: 0.0,
                color: Color32::TRANSPARENT,
            },
            fill: Color32::BLACK,
            stroke: Stroke::default(),
        };
        egui::SidePanel::left("explorer")
            .frame(side_frame)
            .default_width(500.0)
            .min_width(200.0)
            .max_width(500.0)
            .resizable(true)
            .show(ctx, |ui| {
                if let Some(tree) = self.tree.as_mut() {
                    ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| ExplorerResponse::new(tree.ui(ui)));
                }
            });
        ExplorerResponse::new(None)
    }

    pub fn open(&mut self, path: &Path) {
        self.tree = Some(Tree::new(path));
    }
}

pub struct ExplorerResponse {
    opened: Option<PathBuf>,
}

impl ExplorerResponse {
    fn new(opened: Option<PathBuf>) -> Self {
        Self { opened }
    }

    pub(crate) fn opened(&self, callback: impl FnOnce(PathBuf)) {
        if let Some(opened) = self.opened.clone() {
            callback(opened);
        }
    }
}
