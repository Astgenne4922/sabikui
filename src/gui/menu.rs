use crate::gui::{
    actions::Action,
    data::{state::State, table_columns::TableColumns},
};
use egui::{Event, MenuBar, TopBottomPanel};
use std::sync::mpsc;

pub struct Menu {
    message_sender: mpsc::Sender<Action>,
}

impl Menu {
    pub const fn new(message_sender: mpsc::Sender<Action>) -> Self {
        Self { message_sender }
    }

    pub fn show(&self, ctx: &egui::Context, state: &mut State) {
        if state.always_on_top {
            ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::Normal));
        }

        TopBottomPanel::top("top_panel").show(ctx, |ui| self.ui(ui, state));
    }

    fn ui(&self, ui: &mut egui::Ui, state: &mut State) {
        ui.vertical(|ui| {
            self.menu_bar(ui, state);
            self.tool_bar(ui);
        });
    }

    fn menu_bar(&self, ui: &mut egui::Ui, state: &mut State) {
        MenuBar::new().ui(ui, |ui| {
            self.file_menu(ui);
            self.edit_menu(ui, state);
            self.view_menu(ui, state);
            Self::options_menu(ui, state);
            Self::help_menu(ui);
        });
    }

    fn tool_bar(&self, ui: &mut egui::Ui) {
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
                self.message_sender.send(Action::AddFiles(None)).unwrap();
            }

            if ui.button("󰉗").on_hover_text("Add Folder").clicked() {
                self.message_sender.send(Action::AddFolders(None)).unwrap();
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

    fn file_menu(&self, ui: &mut egui::Ui) {
        ui.menu_button("File", |ui| {
            if ui.button("Add File                F2").clicked() {
                self.message_sender.send(Action::AddFiles(None)).unwrap();
            }

            if ui.button("Add Folder              F3").clicked() {
                self.message_sender.send(Action::AddFolders(None)).unwrap();
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

    fn edit_menu(&self, ui: &mut egui::Ui, state: &State) {
        ui.menu_button("Edit", |ui| {
            if ui.button("Copy Selected     CTRL + C").clicked() {
                self.message_sender.send(Action::CopySelected).unwrap();
            }

            if ui.button("Explorer Paste    CTRL + V").clicked() {
                ui.ctx().input(|i| {
                    for event in &i.events {
                        if let Event::Paste(to_paste) = event {
                            self.message_sender.send(Action::Paste(to_paste.clone())).unwrap();
                        }
                    }
                });
            }

            ui.separator();

            ui.menu_button("Copy", |ui| {
                for alg in state.algorithms.keys() {
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

    fn view_menu(&self, ui: &mut egui::Ui, state: &State) {
        ui.menu_button("View", |ui| {
            ui.menu_button("Sort By", |ui| {
                if ui.button("Filename").clicked() {
                    self.message_sender.send(Action::SortBy("Filename".to_owned())).unwrap();
                }

                ui.menu_button("Algorithm", |ui| {
                    for alg in state.algorithms.keys() {
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

    fn options_menu(ui: &mut egui::Ui, state: &mut State) {
        ui.menu_button("Options", |ui| {
            // TODO Options Columns
            ui.menu_button("Choose Columns", |ui| {
                for (column, is_checked) in &mut state.columns {
                    let label = if *is_checked {
                        format!("󰄬  {column}")
                    } else {
                        format!("   {column}")
                    };
                    if ui.button(label).clicked() {
                        *is_checked = !*is_checked;
                        if let TableColumns::Algorithms(alg) = column {
                            for file in &mut state.files {
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

            ui.checkbox(&mut state.always_on_top, "Always on Top");
        });
    }

    fn help_menu(ui: &mut egui::Ui) {
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
}
