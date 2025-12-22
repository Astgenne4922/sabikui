use egui::{ModifierNames, RichText, Ui};

use crate::gui::{
    actions::Action,
    data::{
        constants::{
            labels,
            shortcuts::{self, KEYBINDS},
        },
        state::{OpenWindow, State},
    },
    utils::{open_external_window, pre_render},
};

pub fn menu(ui: &mut Ui, state: &State) -> Vec<Action> {
    let mut events = Vec::new();
    ui.menu_button(labels::HELP_MENU, |ui| {
        if ui.button(labels::ABOUT).clicked() {
            events.push(Action::OpenExternalWindow(
                OpenWindow::About,
                Some(pre_render(ui.ctx(), about)),
            ));
        }
        if ui.button(labels::KEYBINDS).clicked() {
            events.push(Action::OpenExternalWindow(
                OpenWindow::Keybinds,
                Some(pre_render(ui.ctx(), keybinds)),
            ));
        }
    });

    match state.open_extra_window {
        OpenWindow::About => {
            events.extend(open_external_window(
                ui,
                OpenWindow::About,
                state.extra_window_size.unwrap(),
                &|ui| {
                    about(ui);
                    None
                },
            ));
        }
        OpenWindow::Keybinds => {
            events.extend(open_external_window(
                ui,
                OpenWindow::Keybinds,
                state.extra_window_size.unwrap(),
                &|ui| {
                    keybinds(ui);
                    None
                },
            ));
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
    egui::Grid::new(OpenWindow::Keybinds).num_columns(2).show(ui, |ui| {
        ui.heading(labels::COPY_SELECTED);
        ui.heading(shortcuts::COPY_SELECTED.format(&ModifierNames::NAMES, cfg!(target_os = "macos")));
        ui.end_row();

        ui.heading(labels::EXPLORER_PASTE);
        ui.heading(shortcuts::EXPLORER_PASTE.format(&ModifierNames::NAMES, cfg!(target_os = "macos")));
        ui.end_row();

        for (shortcut, action) in KEYBINDS {
            let label = match action {
                Action::Refresh => labels::REFRESH,
                Action::SelectAll => labels::SELECT_ALL,
                Action::DeselectAll => labels::DESELECT_ALL,
                Action::SaveSelected => labels::SAVE_SELECTED,
                Action::ClearSelected => labels::CLEAR_SELECTED,
                Action::AddFiles(_) => labels::ADD_FILE,
                Action::AddFolders(_) => labels::ADD_FOLDER,
                Action::AddWildcard => labels::ADD_WILDCARD,
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
