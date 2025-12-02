use crate::{algorithms, gui::data::hashed_file::HashedFile};
use egui::{MenuBar, TopBottomPanel};
use std::collections::HashMap;

pub struct Menu {
    algorithms: HashMap<String, bool>,
    always_on_top: bool,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            algorithms: algorithms::get_hash_functions()
                .iter()
                .map(|h| (h.to_owned(), true))
                .collect::<HashMap<_, _>>(),
            always_on_top: false,
        }
    }
}

impl Menu {
    pub fn show(&mut self, ctx: &egui::Context, files: &mut [HashedFile]) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.ui(ui, files);
        });
        if self.always_on_top {
            ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::Normal));
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, files: &mut [HashedFile]) {
        ui.vertical(|ui| {
            self.menu_bar(ui, files);
            self.tool_bar(ui, files);
        });
    }

    fn menu_bar(&mut self, ui: &mut egui::Ui, files: &mut [HashedFile]) {
        MenuBar::new().ui(ui, |ui| {
            self.file_menu(ui, files);
            self.edit_menu(ui, files);
            self.view_menu(ui, files);
            self.options_menu(ui, files);
            self.help_menu(ui, files);
        });
    }

    fn tool_bar(&mut self, ui: &mut egui::Ui, files: &mut [HashedFile]) {

        // TODO Toolbar
        // [ ] add file
        // [ ] add folder
        // [ ] add wildcard
        // [ ] clear all

        // [ ] Save selected
        // [ ] Refresh
        // [ ] Copy Selected
    }

    fn file_menu(&mut self, ui: &mut egui::Ui, files: &mut [HashedFile]) {
        // TODO File
        // [ ] add file - F2
        // [ ] add folder - F3
        // [ ] add wildcard - F4

        // [ ] clear all - CTRL+X
        // [ ] clear selected - Del
        // [ ] Save selected - CTRL+S

        // [ ] exit
        ui.menu_button("File", |ui| {});
    }

    fn edit_menu(&mut self, ui: &mut egui::Ui, files: &mut [HashedFile]) {
        // TODO Edit
        // [ ] Copy Selected - CTRL+C
        // [ ] Explorer paste - CTRL+V

        // [ ] Copy [ALG]

        // [ ] select all - CTRL+A
        // [ ] deselect all - CTRL+D
        ui.menu_button("Edit", |ui| {});
    }

    fn view_menu(&mut self, ui: &mut egui::Ui, files: &mut [HashedFile]) {
        // TODO View
        // [ ] Sort by
        //     / Filename
        //     / [ALG]
        //     / Last Edit
        //     / File Size
        //     / Extension
        // [ ] Choose columns
        // [ ] Refresh - F5
        ui.menu_button("View", |ui| {});
    }

    fn options_menu(&mut self, ui: &mut egui::Ui, files: &mut [HashedFile]) {
        ui.menu_button("Options", |ui| {
            ui.menu_button("Algorithms", |ui| {
                // [ ] Algorithms
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

            if ui.button("Highlight identical hashes").clicked() {
                // [ ] Mark identical hashes
            }

            ui.checkbox(&mut self.always_on_top, "Always on Top");
        });
    }

    fn help_menu(&mut self, ui: &mut egui::Ui, files: &mut [HashedFile]) {
        ui.menu_button("Help", |ui| {
            if ui.button("About").clicked() {
                // [ ] About
            }

            if ui.button("Github page").clicked() {
                ui.ctx()
                    .open_url(egui::OpenUrl::new_tab("https://github.com/Astgenne4922/sabikui"));
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
