use egui::{Grid, ModifierNames, RichText, Ui};
use rust_i18n::t;

use crate::gui::{
    actions::{Action, files::FileAction, selection::SelectionAction, window::WindowAction},
    constants::shortcuts::{self, KEYBINDS},
    data::state::{OpenWindow, State},
    utils::open_window,
};

pub fn menu(ui: &mut Ui, state: &State) -> Vec<Action> {
    let mut events = Vec::new();
    ui.menu_button(t!("Help"), |ui| {
        if ui.button(t!("About")).clicked() {
            events.push(Action::Window(WindowAction::OpenExternalWindow(OpenWindow::About)));
        }
        if ui.button(t!("Keybinds")).clicked() {
            events.push(Action::Window(WindowAction::OpenExternalWindow(OpenWindow::Keybinds)));
        }
    });

    match state.open_extra_window {
        OpenWindow::About => {
            events.extend(open_window(ui.ctx(), t!("About").to_string(), true, |ui| {
                about(ui);
                None
            }));
        }
        OpenWindow::Keybinds => {
            events.extend(open_window(ui.ctx(), t!("Keybinds").to_string(), true, |ui| {
                keybinds(ui);
                None
            }));
        }
        _ => {}
    }

    events
}

fn about(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading(format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
        ui.heading(format!("By {} with 󰋑", env!("CARGO_PKG_AUTHORS")));
        ui.hyperlink_to(RichText::new("Github Repo").heading(), env!("CARGO_PKG_REPOSITORY"));
    });
}

fn keybinds(ui: &mut Ui) {
    Grid::new(OpenWindow::Keybinds).num_columns(2).show(ui, |ui| {
        ui.heading(t!("Copy Selected"));
        ui.heading(shortcuts::COPY_SELECTED.format(&ModifierNames::NAMES, cfg!(target_os = "macos")));
        ui.end_row();

        ui.heading(t!("Explorer Paste"));
        ui.heading(shortcuts::EXPLORER_PASTE.format(&ModifierNames::NAMES, cfg!(target_os = "macos")));
        ui.end_row();

        for (shortcut, action) in KEYBINDS {
            let label = match action {
                Action::Files(FileAction::Refresh) => t!("Refresh"),
                Action::Selection(SelectionAction::SelectAll) => t!("Select All"),
                Action::Selection(SelectionAction::DeselectAll) => t!("Deselect All"),
                Action::Selection(SelectionAction::SaveSelected) => t!("Save Selected"),
                Action::Selection(SelectionAction::ClearSelected) => t!("Clear Selected"),
                Action::Files(FileAction::PickFiles) => t!("Add File"),
                Action::Files(FileAction::PickFolders) => t!("Add Folder"),
                Action::Window(WindowAction::OpenExternalWindow(OpenWindow::AddWildcard)) => t!("Add by Wildcard"),
                _ => unreachable!(),
            };
            ui.heading(label);
            ui.heading(shortcut.format(&ModifierNames::NAMES, cfg!(target_os = "macos")));
            ui.end_row();
        }

        ui.heading(t!("Clear All"));
        ui.heading(shortcuts::CLEAR_ALL.format(&ModifierNames::NAMES, cfg!(target_os = "macos")));
        ui.end_row();
    });
}
