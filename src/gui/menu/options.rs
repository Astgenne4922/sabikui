use egui::Ui;

use crate::gui::{
    actions::Action,
    data::{constants::labels, table_columns::TableColumns},
    utils::check_button,
};

pub fn menu(ui: &mut Ui, columns: &[(TableColumns, bool)], always_on_top: bool, mark_same: bool) -> Vec<Action> {
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

        // TODO Mark identical hashes
        if check_button(ui, labels::HIGHLIGHT, mark_same).clicked() {
            events.push(Action::ToggleMarkSame);
        }

        if check_button(ui, labels::ALWAYS_ON_TOP, always_on_top).clicked() {
            events.push(Action::ToggleAlwaysOnTop);
        }

        egui::widgets::global_theme_preference_buttons(ui);
    });

    events
}
