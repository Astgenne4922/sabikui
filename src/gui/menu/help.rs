use egui::{ModifierNames, Ui};

use crate::gui::{
    actions::Action,
    data::{
        constants::{
            labels,
            shortcuts::{self, KEYBINDS},
        },
        state::{OpenWindow, State},
    },
    utils::open_external_window,
};

pub fn menu(ui: &mut Ui, state: &State) {
    ui.menu_button(labels::HELP_MENU, |ui| {
        if ui.button(labels::ABOUT).clicked() {
            *state.open_extra_window.write() = OpenWindow::About;
        }
        if ui.button(labels::KEYBINDS).clicked() {
            *state.open_extra_window.write() = OpenWindow::Keybinds;
        }
    });

    let open_window = *state.open_extra_window.read();
    match open_window {
        OpenWindow::About => {
            let show_deferred_viewport = state.open_extra_window.clone();
            open_external_window(ui, OpenWindow::About, [450.0, 75.0], move |ctx, class| {
                if class == egui::ViewportClass::Deferred {
                    egui::CentralPanel::default().show(ctx, about);

                    if ctx.input(|i| i.viewport().close_requested()) {
                        *show_deferred_viewport.write() = OpenWindow::None;
                    }
                }
            });
        }
        OpenWindow::Keybinds => {
            let show_deferred_viewport = state.open_extra_window.clone();
            open_external_window(ui, OpenWindow::Keybinds, [200.0, 250.0], move |ctx, class| {
                if class == egui::ViewportClass::Deferred {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        egui::Grid::new(OpenWindow::Keybinds).num_columns(2).show(ui, keybinds)
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        *show_deferred_viewport.write() = OpenWindow::None;
                    }
                }
            });
        }
        _ => {}
    }
}

fn about(ui: &mut Ui) {
    ui.vertical_centered_justified(|ui| {
        ui.label(format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
        ui.label(format!("Author: {}", env!("CARGO_PKG_AUTHORS")));
        ui.label(format!("Github Repo: {}", env!("CARGO_PKG_REPOSITORY")));
    });
}

fn keybinds(ui: &mut Ui) {
    ui.label(labels::COPY_SELECTED);
    ui.label(shortcuts::COPY_SELECTED.format(&ModifierNames::NAMES, cfg!(target_os = "macos")));
    ui.end_row();

    ui.label(labels::EXPLORER_PASTE);
    ui.label(shortcuts::EXPLORER_PASTE.format(&ModifierNames::NAMES, cfg!(target_os = "macos")));
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
        ui.label(label);
        ui.label(shortcut.format(&ModifierNames::NAMES, cfg!(target_os = "macos")));
        ui.end_row();
    }

    ui.label(labels::CLEAR_ALL);
    ui.label(shortcuts::CLEAR_ALL.format(&ModifierNames::NAMES, cfg!(target_os = "macos")));
    ui.end_row();
}
