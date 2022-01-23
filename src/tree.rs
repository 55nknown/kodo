use eframe::egui::{CollapsingHeader, Ui};

#[derive(Clone, Default)]
pub struct Tree(String, SubTree);

impl Tree {
    pub fn demo() -> Self {
        Self(
            String::from("root"),
            SubTree(vec![
                SubTree(vec![SubTree::default(); 4]),
                SubTree(vec![SubTree(vec![SubTree::default(); 2]); 3]),
            ]),
        )
    }
    pub fn ui(&mut self, ui: &mut Ui) {
        self.1.ui(ui, 0, "root", &mut self.0)
    }
}

#[derive(Clone, Default)]
struct SubTree(Vec<SubTree>);

impl SubTree {
    pub fn ui(&mut self, ui: &mut Ui, depth: usize, name: &str, selected_name: &mut String) {
        let response = CollapsingHeader::new(name)
            .default_open(depth < 1)
            .selectable(true)
            .selected(selected_name.as_str() == name)
            .show(ui, |ui| self.children_ui(ui, name, depth, selected_name));
        if response.header_response.clicked() {
            *selected_name = name.to_string();
        }
    }

    fn children_ui(
        &mut self,
        ui: &mut Ui,
        parent_name: &str,
        depth: usize,
        selected_name: &mut String,
    ) {
        self.0 = std::mem::take(self)
            .0
            .into_iter()
            .enumerate()
            .map(|(i, mut tree)| {
                tree.ui(
                    ui,
                    depth + 1,
                    &format!("{}/{}", parent_name, i),
                    selected_name,
                );
                tree
            })
            .collect();
    }
}
