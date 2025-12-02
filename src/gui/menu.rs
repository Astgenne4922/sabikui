use crate::{algorithms, gui::data::hashed_file::HashedFile};
use egui::{MenuBar, TopBottomPanel};
use std::collections::HashMap;

/*
Menubar
TODO File
    [ ] add file - F2
    [ ] add folder - F3
    [ ] add wildcard - F4

    [ ] clear all - CTRL+X
    [ ] clear selected - Del
    [ ] Save selected - CTRL+S

    [ ] exit
TODO Edit
    [ ] Copy Selected - CTRL+C
    [ ] Explorer paste - CTRL+V

    [ ] Copy [ALG]

    [ ] select all - CTRL+A
    [ ] deselect all - CTRL+D
TODO View
    [ ] Sort by
        / Filename
        / [ALG]
        / Last Edit
        / File Size
        / Extension
    [ ] Choose columns
    [ ] Refresh - F5
TODO Options
    [ ] Algorithms
    [ ] Mark identical hashes
    [ ] Always om top
TODO Help
    [ ] About
    [ ] Github

Toolbar
    [ ] add file
    [ ] add folder
    [ ] add wildcard
    [ ] clear all

    [ ] Save selected
    [ ] Refresh
    [ ] Copy Selected
*/

pub struct Menu {
    algorithms: HashMap<String, bool>,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            algorithms: algorithms::get_hash_functions()
                .iter()
                .map(|h| (h.to_owned(), true))
                .collect::<HashMap<_, _>>(),
        }
    }
}

impl Menu {
    pub fn show(&mut self, ctx: &egui::Context, files: &mut [HashedFile]) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                self.ui(ui, files);
            });
        });
    }

    fn ui(&mut self, ui: &mut egui::Ui, files: &mut [HashedFile]) {
        ui.menu_button("Algorithms", |ui| {
            for (alg, is_checked) in &mut self.algorithms {
                if ui.checkbox(is_checked, alg.to_string()).changed() {
                    for file in files.iter_mut() {
                        if *is_checked {
                            file.add_digest_for(alg);
                        } else {
                            file.remove_digest(alg);
                        }
                    }
                }
            }
        });
    }

    pub fn algorithm_list(&self) -> Vec<String> {
        let mut active_algorithms = self
            .algorithms
            .iter()
            .filter_map(|(alg, is_checked)| if *is_checked { Some(alg.clone()) } else { None })
            .collect::<Vec<_>>();
        active_algorithms.sort();
        active_algorithms
    }
}
