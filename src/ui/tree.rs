#![allow(dead_code)]
use eframe::egui::{CollapsingHeader, Ui};
use std::{
    cmp::Ordering,
    path::{Path, PathBuf},
    vec,
};

pub struct Tree {
    // root: PathBuf,
    selected: PathBuf,
    subtree: SubTree,
}

impl Tree {
    pub fn new(root: &Path) -> Self {
        Self {
            // root: root.to_path_buf(),
            selected: root.to_path_buf(),
            subtree: SubTree::new(root),
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Option<PathBuf> {
        self.subtree.ui(ui, 0, &mut self.selected)
    }
}

struct SubTree {
    parent: PathBuf,
    children: Vec<SubTree>,
}

impl SubTree {
    pub fn new(parent: &Path) -> Self {
        let children = Self::build(parent);

        Self {
            parent: parent.to_path_buf(),
            children,
        }
    }

    fn sort(children: &mut Vec<SubTree>) {
        children.sort_by(|a, b| {
            if a.parent.is_dir() && b.parent.is_file() {
                Ordering::Less
            } else if b.parent.is_dir() && a.parent.is_file() {
                Ordering::Greater
            } else {
                a.get_name().cmp(&b.get_name())
            }
        });
    }

    fn build(parent: &Path) -> Vec<SubTree> {
        let mut children = vec![];

        if parent.is_dir() {
            if let Ok(dir) = std::fs::read_dir(parent) {
                for child in dir.flatten() {
                    children.push(SubTree {
                        parent: child.path(),
                        children: Self::build(child.path().as_path()),
                    });
                }
            }
        }

        Self::sort(&mut children);

        children
    }

    pub fn get_name(&self) -> String {
        self.parent
            .file_name()
            .unwrap_or_default()
            .to_os_string()
            .into_string()
            .unwrap_or_default()
    }

    pub fn ui(&mut self, ui: &mut Ui, depth: usize, selected: &mut PathBuf) -> Option<PathBuf> {
        if self.parent.is_dir() {
            let response = CollapsingHeader::new(self.get_name())
                .default_open(depth < 1)
                .selectable(true)
                .selected(self.parent == *selected)
                .show(ui, |ui| self.children_ui(ui, depth, selected));
            if response.header_response.clicked() {
                *selected = self.parent.clone();
            }
            return response.body_returned.flatten();
        } else if self.parent.is_file() {
            if ui.button(self.get_name()).clicked() {
                *selected = self.parent.clone();
                return Some(self.parent.clone());
            }
        }

        None
    }

    fn children_ui(
        &mut self,
        ui: &mut Ui,
        depth: usize,
        selected: &mut PathBuf,
    ) -> Option<PathBuf> {
        for tree in &mut self.children {
            if let Some(opened) = tree.ui(ui, depth + 1, selected) {
                return Some(opened);
            }
        }

        None
    }
}
