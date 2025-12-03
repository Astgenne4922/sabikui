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
        ui.horizontal(|ui| {
            ui.style_mut().override_font_id = Some(egui::FontId::proportional(24.0));

            if ui.button("󰆓").on_hover_text("Save Selected").clicked() {
                // [ ] Save selected
            }

            if ui.button("󰑐").on_hover_text("Refresh").clicked() {
                // [ ] Refresh
            }

            ui.separator();

            if ui.button("").on_hover_text("Add File").clicked() {
                // [ ] add file
            }

            if ui.button("󰉗").on_hover_text("Add Folder").clicked() {
                // [ ] add folder
            }

            if ui.button("").on_hover_text("Add by Wildcard").clicked() {
                // [ ] add wildcard
            }

            ui.separator();

            if ui.button("󰾂").on_hover_text("Select All").clicked() {
                // [ ] Select all
            }

            if ui.button("󰆏").on_hover_text("Copy Selected").clicked() {
                // [ ] Copy Selected
            }

            if ui.button("󰗨").on_hover_text("Clear All").clicked() {
                // [ ] clear all
            }
        });
    }

    fn file_menu(&mut self, ui: &mut egui::Ui, files: &mut [HashedFile]) {
        // TODO File
        ui.menu_button("File", |ui| {
            if ui.button("Add File                F2").clicked() {
                // [ ] add file - F2
            }

            if ui.button("Add Folder              F3").clicked() {
                // [ ] add folder - F3
            }

            if ui.button("Add by Wildcard         F4").clicked() {
                // [ ] add wildcard - F4
            }

            ui.separator();

            if ui.button("Save Selected     CTRL + S").clicked() {
                // [ ] Save selected - CTRL+S
            }

            ui.separator();

            if ui.button("Clear All         CTRL + X").clicked() {
                // [ ] clear all - CTRL+X
            }

            if ui.button("Clear Selected         Del").clicked() {
                // [ ] clear selected - Del
            }

            ui.separator();

            if ui.button("Exit").clicked() {
                // [ ] exit
            }
        });
    }

    fn edit_menu(&mut self, ui: &mut egui::Ui, files: &mut [HashedFile]) {
        // TODO Edit
        ui.menu_button("Edit", |ui| {
            if ui.button("Copy Selected     CTRL + C").clicked() {
                // [ ] Copy Selected - CTRL+C
            }

            if ui.button("Explorer Paste    CTRL + V").clicked() {
                // [ ] Explorer paste - CTRL+V
            }

            ui.separator();

            ui.menu_button("Copy", |ui| {
                for (alg, _) in &mut self.algorithms {
                    // [ ] Copy [ALG]
                    if ui.button(alg.to_ascii_uppercase()).clicked() {}
                }
            });

            ui.separator();

            if ui.button("Select All        CTRL + A").clicked() {
                // [ ] select all - CTRL+A
            }

            if ui.button("Deselect All      CTRL + D").clicked() {
                // [ ] deselect all - CTRL+D
            }
        });
    }

    fn view_menu(&mut self, ui: &mut egui::Ui, files: &mut [HashedFile]) {
        ui.menu_button("View", |ui| {
            // TODO View

            ui.menu_button("Sort By", |ui| {
                // [ ] Sort by

                if ui.button("Filename").clicked() {
                    //     / Filename
                }

                ui.menu_button("Algorithm", |ui| {
                    //     / [ALG]
                });

                if ui.button("Edit Time").clicked() {
                    //     / Edit Time
                }

                if ui.button("File Size").clicked() {
                    //     / File Size
                }

                if ui.button("Extension").clicked() {
                    //     / Extension
                }
            });

            if ui.button("Refresh       F5").clicked() {
                // [ ] Refresh - F5
            }
        });
    }

    fn options_menu(&mut self, ui: &mut egui::Ui, files: &mut [HashedFile]) {
        ui.menu_button("Options", |ui| {
            ui.menu_button("Choose Columns", |ui| {
                if ui.button("Filename").clicked() {
                    //     / Filename
                }

                ui.menu_button("Algorithms", |ui| {
                    // [ ] Algorithms
                    for (alg, is_checked) in &mut self.algorithms {
                        let label = if *is_checked {
                            format!("󰄬  {alg}")
                        } else {
                            format!("   {alg}")
                        };
                        if ui.button(label).clicked() {
                            *is_checked = !*is_checked;
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

                if ui.button("Edit Time").clicked() {
                    //     / Edit Time
                }

                if ui.button("File Size").clicked() {
                    //     / File Size
                }

                if ui.button("Extension").clicked() {
                    //     / Extension
                }
            });

            if ui.button("Highlight identical Hashes").clicked() {
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
