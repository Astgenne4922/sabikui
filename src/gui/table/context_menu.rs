use egui::{Event, Response};

use crate::gui::{
    actions::Action,
    data::{
        constants::{labels, shortcuts},
        table_columns::TableColumns,
    },
    utils::{long_submenu, shortcut_button},
};

pub fn open(response: &Response, are_row_selected: bool, active_columns: &[TableColumns]) -> Vec<Action> {
    let mut events = Vec::new();

    response.context_menu(|ui| {
        ui.add_enabled_ui(are_row_selected, |ui| {
            if shortcut_button(ui, labels::SAVE_SELECTED, shortcuts::SAVE_SELECTED).clicked() {
                events.push(Action::SaveSelected);
            }

            if shortcut_button(ui, labels::COPY_SELECTED, shortcuts::COPY_SELECTED).clicked() {
                events.push(Action::CopySelected);
            }

            long_submenu(ui, labels::COPY, active_columns, |ui, col| {
                if ui.button(col.to_string()).clicked() {
                    events.push(Action::CopyProperty(col.clone()));
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
