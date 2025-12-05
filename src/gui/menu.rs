use crate::{
    algorithms,
    gui::{
        actions::Action,
        data::{hashed_file::HashedFile, table_columns::TableColumns},
    },
};
use egui::{MenuBar, TopBottomPanel};
use std::{collections::HashMap, sync::mpsc};

pub struct Menu {
    pub columns: Vec<(TableColumns, bool)>,
    algorithms: HashMap<String, bool>,
    always_on_top: bool,
    message_sender: mpsc::Sender<Action>,
}

impl Menu {
    pub fn new(message_sender: mpsc::Sender<Action>) -> Self {
        let mut cols = vec![(TableColumns::Path, true), (TableColumns::FileName, true)];
        cols.extend(
            algorithms::get_hash_functions()
                .iter()
                .map(|h| (TableColumns::Algorithms(h.to_owned()), true)),
        );
        cols.push((TableColumns::FileSize, true));
        cols.push((TableColumns::LastEdit, true));
        cols.push((TableColumns::Extension, true));
        Self {
            columns: cols,
            algorithms: algorithms::get_hash_functions()
                .iter()
                .map(|h| (h.to_owned(), true))
                .collect::<HashMap<_, _>>(),
            always_on_top: false,
            message_sender,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, files: &mut [HashedFile]) {
        if self.always_on_top {
            ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::Normal));
        }

        TopBottomPanel::top("top_panel").show(ctx, |ui| self.ui(ui, files));
    }

    fn ui(&mut self, ui: &mut egui::Ui, files: &mut [HashedFile]) {
        ui.vertical(|ui| {
            self.menu_bar(ui, files);
            self.tool_bar(ui);
        });
    }

    fn menu_bar(&mut self, ui: &mut egui::Ui, files: &mut [HashedFile]) {
        MenuBar::new().ui(ui, |ui| {
            self.file_menu(ui);
            self.edit_menu(ui);
            self.view_menu(ui);
            self.options_menu(ui, files);
            self.help_menu(ui);
        });
    }

    fn tool_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.style_mut().override_font_id = Some(egui::FontId::proportional(24.0));

            if ui.button("󰆓").on_hover_text("Save Selected").clicked() {
                self.message_sender.send(Action::SaveSelected).unwrap();
            }

            if ui.button("󰑐").on_hover_text("Refresh").clicked() {
                self.message_sender.send(Action::Refresh).unwrap();
            }

            ui.separator();

            if ui.button("").on_hover_text("Add File").clicked() {
                self.message_sender.send(Action::AddFile).unwrap();
            }

            if ui.button("󰉗").on_hover_text("Add Folder").clicked() {
                self.message_sender.send(Action::AddFolder).unwrap();
            }

            if ui.button("").on_hover_text("Add by Wildcard").clicked() {
                self.message_sender.send(Action::AddWildcard).unwrap();
            }

            ui.separator();

            if ui.button("󰾂").on_hover_text("Select All").clicked() {
                self.message_sender.send(Action::SelectAll).unwrap();
            }

            if ui.button("󰆏").on_hover_text("Copy Selected").clicked() {
                self.message_sender.send(Action::ClearSelected).unwrap();
            }

            if ui.button("󰗨").on_hover_text("Clear All").clicked() {
                self.message_sender.send(Action::ClearAll).unwrap();
            }
        });
    }

    fn file_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("File", |ui| {
            if ui.button("Add File                F2").clicked() {
                self.message_sender.send(Action::AddFile).unwrap();
            }

            if ui.button("Add Folder              F3").clicked() {
                self.message_sender.send(Action::AddFolder).unwrap();
            }

            if ui.button("Add by Wildcard         F4").clicked() {
                self.message_sender.send(Action::AddWildcard).unwrap();
            }

            ui.separator();

            if ui.button("Save Selected     CTRL + S").clicked() {
                self.message_sender.send(Action::SaveSelected).unwrap();
            }

            ui.separator();

            if ui.button("Clear All         CTRL + X").clicked() {
                self.message_sender.send(Action::ClearAll).unwrap();
            }

            if ui.button("Clear Selected         Del").clicked() {
                self.message_sender.send(Action::ClearSelected).unwrap();
            }

            ui.separator();

            if ui.button("Exit").clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }

    fn edit_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Edit", |ui| {
            if ui.button("Copy Selected     CTRL + C").clicked() {
                self.message_sender.send(Action::CopySelected).unwrap();
            }

            if ui.button("Explorer Paste    CTRL + V").clicked() {
                self.message_sender.send(Action::Paste).unwrap();
            }

            ui.separator();

            ui.menu_button("Copy", |ui| {
                for (alg, _) in &mut self.algorithms {
                    if ui.button(alg.to_ascii_uppercase()).clicked() {
                        self.message_sender.send(Action::CopyHash(alg.to_owned())).unwrap();
                    }
                }
            });

            ui.separator();

            if ui.button("Select All        CTRL + A").clicked() {
                self.message_sender.send(Action::SelectAll).unwrap();
            }

            if ui.button("Deselect All      CTRL + D").clicked() {
                self.message_sender.send(Action::DeselectAll).unwrap();
            }
        });
    }

    fn view_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("View", |ui| {
            ui.menu_button("Sort By", |ui| {
                if ui.button("Filename").clicked() {
                    self.message_sender.send(Action::SortBy("Filename".to_owned())).unwrap();
                }

                ui.menu_button("Algorithm", |ui| {
                    for (alg, _) in &self.algorithms {
                        if ui.button(alg.to_ascii_uppercase()).clicked() {
                            self.message_sender.send(Action::SortBy(alg.to_owned())).unwrap();
                        }
                    }
                });

                if ui.button("Edit Time").clicked() {
                    self.message_sender
                        .send(Action::SortBy("Edit Time".to_owned()))
                        .unwrap();
                }

                if ui.button("File Size").clicked() {
                    self.message_sender
                        .send(Action::SortBy("File Size".to_owned()))
                        .unwrap();
                }

                if ui.button("Extension").clicked() {
                    self.message_sender
                        .send(Action::SortBy("Extension".to_owned()))
                        .unwrap();
                }
            });

            if ui.button("Refresh       F5").clicked() {
                self.message_sender.send(Action::Refresh).unwrap();
            }
        });
    }

    fn options_menu(&mut self, ui: &mut egui::Ui, files: &mut [HashedFile]) {
        ui.menu_button("Options", |ui| {
            // TODO Options Columns
            ui.menu_button("Choose Columns", |ui| {
                for (column, is_checked) in &mut self.columns {
                    let label = if *is_checked {
                        format!("󰄬  {}", column.to_string())
                    } else {
                        format!("   {}", column.to_string())
                    };
                    if ui.button(label).clicked() {
                        *is_checked = !*is_checked;
                        if let TableColumns::Algorithms(alg) = column {
                            for file in files.iter_mut() {
                                if *is_checked {
                                    file.add_digest_for(alg);
                                } else {
                                    file.remove_digest(alg);
                                }
                            }
                        }
                    }
                }
            });

            if ui.button("Highlight identical Hashes").clicked() {
                // TODO Mark identical hashes
            }

            ui.checkbox(&mut self.always_on_top, "Always on Top");
        });
    }

    fn help_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Help", |ui| {
            if ui.button("About").clicked() {
                // TODO About
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
