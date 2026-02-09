use egui::Ui;
use rust_i18n::t;

use crate::gui::{
    actions::{Action, files::FileAction, selection::SelectionAction, window::WindowAction},
    constants::shortcuts,
    data::state::OpenWindow,
    utils::shortcut_button,
};

pub fn menu(ui: &mut Ui) -> Vec<Action> {
    let mut events = Vec::new();

    ui.menu_button(t!("File"), |ui| {
        if shortcut_button(ui, t!("Add File"), shortcuts::ADD_FILE).clicked() {
            events.push(Action::Files(FileAction::PickFiles));
        }

        if shortcut_button(ui, t!("Add Folder"), shortcuts::ADD_FOLDER).clicked() {
            events.push(Action::Files(FileAction::PickFolders));
        }

        if shortcut_button(ui, t!("Add by Wildcard"), shortcuts::ADD_WILDCARD).clicked() {
            events.push(Action::Window(WindowAction::OpenExternalWindow(
                OpenWindow::AddWildcard,
            )));
        }

        ui.separator();

        if shortcut_button(ui, t!("Save Selected"), shortcuts::SAVE_SELECTED).clicked() {
            events.push(Action::Selection(SelectionAction::SaveSelected));
        }

        ui.separator();

        if shortcut_button(ui, t!("Clear All"), shortcuts::CLEAR_ALL).clicked() {
            events.push(Action::Selection(SelectionAction::ClearAll));
        }

        if shortcut_button(ui, t!("Clear Selected"), shortcuts::CLEAR_SELECTED).clicked() {
            events.push(Action::Selection(SelectionAction::ClearSelected));
        }

        ui.separator();

        if ui.button(t!("Exit")).clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
    });

    events
}
