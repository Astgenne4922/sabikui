use egui::{Event, Response};

use crate::gui::{
    actions::Action,
    data::{
        constants::{labels, shortcuts},
        hashed_file::HashFunction,
    },
    shortcut_button,
};

pub fn open(response: &Response, algorithm_list: &[HashFunction], are_row_selected: bool) -> Vec<Action> {
    let mut events = Vec::new();

    response.context_menu(|ui| {
        ui.add_enabled_ui(are_row_selected, |ui| {
            if shortcut_button(ui, labels::SAVE_SELECTED, shortcuts::SAVE_SELECTED).clicked() {
                events.push(Action::SaveSelected);
            }

            if shortcut_button(ui, labels::COPY_SELECTED, shortcuts::COPY_SELECTED).clicked() {
                events.push(Action::CopySelected);
            }

            ui.menu_button(labels::COPY, |ui| {
                for alg in algorithm_list {
                    if ui.button(alg.to_ascii_uppercase()).clicked() {
                        events.push(Action::CopyHash(alg.clone()));
                    }
                }
            });
        });

        ui.separator();

        if shortcut_button(ui, labels::EXPLORER_PASTE, shortcuts::EXPLORER_PASTE).clicked() {
            ui.ctx().input(|i| {
                for event in &i.events {
                    if let Event::Paste(to_paste) = event {
                        events.push(Action::Paste(to_paste.clone()));
                    }
                }
            });
        }

        ui.separator();

        if shortcut_button(ui, labels::REFRESH, shortcuts::REFRESH).clicked() {
            events.push(Action::Refresh);
        }
    });

    events
}
