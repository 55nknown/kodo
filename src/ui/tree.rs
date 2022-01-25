use eframe::egui::{CollapsingHeader, Ui};
use std::{
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

    pub fn ui(&mut self, ui: &mut Ui) {
        self.subtree.ui(ui, 0, &mut self.selected);
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

    pub fn ui(&mut self, ui: &mut Ui, depth: usize, selected: &mut PathBuf) {
        let response = CollapsingHeader::new(self.get_name())
            .default_open(depth < 1)
            .selectable(true)
            .selected(self.parent == *selected)
            .show(ui, |ui| self.children_ui(ui, depth, selected));
        if response.header_response.clicked() {
            *selected = self.parent.clone();
        }
    }

    fn children_ui(&mut self, ui: &mut Ui, depth: usize, selected: &mut PathBuf) {
        self.children.iter_mut().for_each(|tree| {
            tree.ui(ui, depth + 1, selected);
        });
    }
}
