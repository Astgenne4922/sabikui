use egui::{Event, Ui};

use crate::gui::{
    actions::Action,
    constants::{labels, shortcuts},
    data::table_columns::TableColumns,
    utils::{long_submenu, shortcut_button},
};

pub fn menu(ui: &mut Ui, active_columns: &[TableColumns]) -> Vec<Action> {
    let mut events = Vec::new();

    ui.menu_button(labels::EDIT_MENU, |ui| {
        if shortcut_button(ui, labels::COPY_SELECTED, shortcuts::COPY_SELECTED).clicked() {
            events.push(Action::CopySelected);
        }

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

        long_submenu(ui, labels::COPY, active_columns, |ui, col| {
            if ui.button(col.to_string()).clicked() {
                events.push(Action::CopyProperty(col.clone()));
            }
        });

        ui.separator();

        if shortcut_button(ui, labels::SELECT_ALL, shortcuts::SELECT_ALL).clicked() {
            events.push(Action::SelectAll);
        }

        if shortcut_button(ui, labels::DESELECT_ALL, shortcuts::DESELECT_ALL).clicked() {
            events.push(Action::DeselectAll);
        }
    });

    events
}
