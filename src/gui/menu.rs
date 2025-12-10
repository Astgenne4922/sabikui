use crate::gui::{
    actions::Action,
    data::{
        constants::{icons, labels, shortcuts},
        state::State,
        table_columns::TableColumns,
    },
    shortcut_button,
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

            let mut events = Vec::new();

            if ui
                .button(icons::SAVE_SELECTED)
                .on_hover_text(labels::SAVE_SELECTED)
                .clicked()
            {
                events.push(Action::SaveSelected);
            }

            if ui.button(icons::REFRESH).on_hover_text(labels::REFRESH).clicked() {
                events.push(Action::Refresh);
            }

            ui.separator();

            if ui.button(icons::ADD_FILE).on_hover_text(labels::ADD_FILE).clicked() {
                events.push(Action::AddFiles(None));
            }

            if ui.button(icons::ADD_FOLDER).on_hover_text(labels::ADD_FOLDER).clicked() {
                events.push(Action::AddFolders(None));
            }

            if ui
                .button(icons::ADD_WILDCARD)
                .on_hover_text(labels::ADD_WILDCARD)
                .clicked()
            {
                events.push(Action::AddWildcard);
            }

            ui.separator();

            if ui.button(icons::SELECT_ALL).on_hover_text(labels::SELECT_ALL).clicked() {
                events.push(Action::SelectAll);
            }

            if ui
                .button(icons::COPY_SELECTED)
                .on_hover_text(labels::COPY_SELECTED)
                .clicked()
            {
                events.push(Action::ClearSelected);
            }

            if ui.button(icons::CLEAR_ALL).on_hover_text(labels::CLEAR_ALL).clicked() {
                events.push(Action::ClearAll);
            }

            for event in events {
                self.message_sender
                    .send(event)
                    .expect("The receiver should always be available");
            }
        });
    }

    fn file_menu(&self, ui: &mut egui::Ui) {
        ui.menu_button(labels::FILE_MENU, |ui| {
            let mut events = Vec::new();

            if shortcut_button(ui, labels::ADD_FILE, shortcuts::ADD_FILE).clicked() {
                events.push(Action::AddFiles(None));
            }

            if shortcut_button(ui, labels::ADD_FOLDER, shortcuts::ADD_FOLDER).clicked() {
                events.push(Action::AddFolders(None));
            }

            if shortcut_button(ui, labels::ADD_WILDCARD, shortcuts::ADD_WILDCARD).clicked() {
                events.push(Action::AddWildcard);
            }

            ui.separator();

            if shortcut_button(ui, labels::SAVE_SELECTED, shortcuts::SAVE_SELECTED).clicked() {
                events.push(Action::SaveSelected);
            }

            ui.separator();

            if shortcut_button(ui, labels::CLEAR_ALL, shortcuts::CLEAR_ALL).clicked() {
                events.push(Action::ClearAll);
            }

            if shortcut_button(ui, labels::CLEAR_SELECTED, shortcuts::CLEAR_SELECTED).clicked() {
                events.push(Action::ClearSelected);
            }

            ui.separator();

            if ui.button(labels::EXIT).clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }

            for event in events {
                self.message_sender
                    .send(event)
                    .expect("The receiver should always be available");
            }
        });
    }

    fn edit_menu(&self, ui: &mut egui::Ui, state: &State) {
        ui.menu_button(labels::EDIT_MENU, |ui| {
            let mut events = Vec::new();

            if shortcut_button(ui, labels::COPY_SELECTED, shortcuts::COPY_SELECTED).clicked() {
                events.push(Action::CopySelected);
            }

            if shortcut_button(ui, labels::EXPLORER_PASTE, shortcuts::EXPLORER_PASTE).clicked() {
                ui.ctx().input(|i| {
                    for event in &i.events {
                        if let Event::Paste(to_paste) = event {
                            events.push(Action::Paste(to_paste.to_owned()));
                        }
                    }
                });
            }

            ui.separator();

            ui.menu_button(labels::COPY, |ui| {
                for alg in state.algorithm_list() {
                    if ui.button(alg.to_ascii_uppercase()).clicked() {
                        events.push(Action::CopyHash(alg));
                    }
                }
            });

            ui.separator();

            if shortcut_button(ui, labels::SELECT_ALL, shortcuts::SELECT_ALL).clicked() {
                events.push(Action::SelectAll);
            }

            if shortcut_button(ui, labels::DESELECT_ALL, shortcuts::DESELECT_ALL).clicked() {
                events.push(Action::DeselectAll);
            }

            for event in events {
                self.message_sender
                    .send(event)
                    .expect("The receiver should always be available");
            }
        });
    }

    fn view_menu(&self, ui: &mut egui::Ui, state: &State) {
        let make_label = |column: &TableColumns| {
            if let Some((col, is_reversed)) = &state.sorting_column
                && col == column
            {
                if *is_reversed {
                    format!("{} {column}", icons::DESCENDING)
                } else {
                    format!("{} {column}", icons::ASCENDING)
                }
            } else {
                format!("  {column}")
            }
        };

        ui.menu_button(labels::VIEW_MENU, |ui| {
            let mut events = Vec::new();

            ui.menu_button(labels::SORT_BY, |ui| {
                if ui.button(make_label(&TableColumns::FileName)).clicked() {
                    events.push(Action::SortBy(TableColumns::FileName));
                }

                if ui.button(make_label(&TableColumns::Path)).clicked() {
                    events.push(Action::SortBy(TableColumns::Path));
                }

                ui.menu_button(labels::ALGORITHM, |ui| {
                    for alg in state.algorithm_list() {
                        if ui.button(make_label(&TableColumns::Algorithms(alg.clone()))).clicked() {
                            events.push(Action::SortBy(TableColumns::Algorithms(alg)));
                        }
                    }
                });

                if ui.button(make_label(&TableColumns::LastEdit)).clicked() {
                    events.push(Action::SortBy(TableColumns::LastEdit));
                }

                if ui.button(make_label(&TableColumns::FileSize)).clicked() {
                    events.push(Action::SortBy(TableColumns::FileSize));
                }

                if ui.button(make_label(&TableColumns::Extension)).clicked() {
                    events.push(Action::SortBy(TableColumns::Extension));
                }
            });

            if shortcut_button(ui, labels::REFRESH, shortcuts::REFRESH).clicked() {
                events.push(Action::Refresh);
            }

            for event in events {
                self.message_sender
                    .send(event)
                    .expect("The receiver should always be available");
            }
        });
    }

    fn options_menu(ui: &mut egui::Ui, state: &mut State) {
        ui.menu_button(labels::OPTIONS_MENU, |ui| {
            // TODO Options Columns
            ui.menu_button(labels::CHOOSE_COLUMNS, |ui| {
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

            if ui.button(labels::HIGHLIGHT).clicked() {
                // TODO Mark identical hashes
            }

            ui.checkbox(&mut state.always_on_top, labels::ALWAYS_ON_TOP);
        });
    }

    fn help_menu(ui: &mut egui::Ui) {
        ui.menu_button(labels::HELP_MENU, |ui| {
            if ui.button(labels::ABOUT).clicked() {
                // TODO About
            }

            if ui.button(labels::GITHUB).clicked() {
                ui.ctx()
                    .open_url(egui::OpenUrl::new_tab("https://github.com/Astgenne4922/sabikui"));
            }
        });
    }
}
