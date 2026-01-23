use egui::Ui;

use crate::gui::{
    actions::Action,
    constants::{labels, shortcuts},
    utils::shortcut_button,
};

pub fn menu(ui: &mut Ui) -> Vec<Action> {
    let mut events = Vec::new();

    ui.menu_button(labels::FILE_MENU, |ui| {
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
    });

    events
}
