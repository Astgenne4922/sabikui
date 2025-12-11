use egui::{Button, Response, Ui};

use crate::gui::{
    actions::Action,
    data::{constants::labels, table_columns::TableColumns},
};

pub fn menu(ui: &mut Ui, columns: &[(TableColumns, bool)], always_on_top: bool) -> Vec<Action> {
    let mut events = Vec::new();

    ui.menu_button(labels::OPTIONS_MENU, |ui| {
        // TODO Options Columns
        ui.menu_button(labels::CHOOSE_COLUMNS, |ui| {
            for (column, is_checked) in columns {
                if check_button(ui, &column.to_string(), *is_checked).clicked() {
                    events.push(Action::ToggleColumn(column.clone()));
                }
            }
        });

        if ui.button(labels::HIGHLIGHT).clicked() {
            // TODO Mark identical hashes
        }

        if check_button(ui, labels::ALWAYS_ON_TOP, always_on_top).clicked() {
            events.push(Action::ToggleAlwaysOnTop);
        }
    });

    events
}

fn check_button(ui: &mut Ui, label: &str, is_checked: bool) -> Response {
    let check = if is_checked { "󰄬" } else { "" };
    ui.add(Button::new(label).right_text(check))
}
