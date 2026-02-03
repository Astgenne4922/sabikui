use egui::{Grid, ModifierNames, RichText, Ui};

use crate::gui::{
    actions::{Action, files::FileAction, selection::SelectionAction, window::WindowAction},
    constants::{
        labels,
        shortcuts::{self, KEYBINDS},
    },
    data::state::{OpenWindow, State},
    utils::open_window,
};

pub fn menu(ui: &mut Ui, state: &State) -> Vec<Action> {
    let mut events = Vec::new();
    ui.menu_button(labels::HELP_MENU, |ui| {
        if ui.button(labels::ABOUT).clicked() {
            events.push(Action::Window(WindowAction::OpenExternalWindow(OpenWindow::About)));
        }
        if ui.button(labels::KEYBINDS).clicked() {
            events.push(Action::Window(WindowAction::OpenExternalWindow(OpenWindow::Keybinds)));
        }
    });

    match state.open_extra_window {
        OpenWindow::About => {
            events.extend(open_window(ui.ctx(), labels::ABOUT.to_string(), true, |ui| {
                about(ui);
                None
            }));
        }
        OpenWindow::Keybinds => {
            events.extend(open_window(ui.ctx(), labels::KEYBINDS.to_string(), true, |ui| {
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
        ui.heading(labels::COPY_SELECTED);
        ui.heading(shortcuts::COPY_SELECTED.format(&ModifierNames::NAMES, cfg!(target_os = "macos")));
        ui.end_row();

        ui.heading(labels::EXPLORER_PASTE);
        ui.heading(shortcuts::EXPLORER_PASTE.format(&ModifierNames::NAMES, cfg!(target_os = "macos")));
        ui.end_row();

        for (shortcut, action) in KEYBINDS {
            let label = match action {
                Action::Files(FileAction::Refresh) => labels::REFRESH,
                Action::Selection(SelectionAction::SelectAll) => labels::SELECT_ALL,
                Action::Selection(SelectionAction::DeselectAll) => labels::DESELECT_ALL,
                Action::Selection(SelectionAction::SaveSelected) => labels::SAVE_SELECTED,
                Action::Selection(SelectionAction::ClearSelected) => labels::CLEAR_SELECTED,
                Action::Files(FileAction::PickFiles) => labels::ADD_FILE,
                Action::Files(FileAction::PickFolders) => labels::ADD_FOLDER,
                Action::Window(WindowAction::OpenExternalWindow(OpenWindow::AddWildcard)) => labels::ADD_WILDCARD,
                _ => unreachable!(),
            };
            ui.heading(label);
            ui.heading(shortcut.format(&ModifierNames::NAMES, cfg!(target_os = "macos")));
            ui.end_row();
        }

        ui.heading(labels::CLEAR_ALL);
        ui.heading(shortcuts::CLEAR_ALL.format(&ModifierNames::NAMES, cfg!(target_os = "macos")));
        ui.end_row();
    });
}
