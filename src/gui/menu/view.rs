use egui::Ui;

use crate::gui::{
    actions::{Action, files::FileAction, options::OptionsAction},
    constants::shortcuts,
    data::table_columns::TableColumns,
    utils::{long_submenu, shortcut_button, sort_button},
};
use rust_i18n::t;

pub fn menu(
    ui: &mut Ui, active_columns: &[TableColumns], sorting_column: Option<&(TableColumns, bool)>,
) -> Vec<Action> {
    let mut events = Vec::new();

    ui.menu_button(t!("View"), |ui| {
        long_submenu(ui, t!("Sort By"), active_columns, |ui, col| {
            if sort_button(ui, col, sorting_column).clicked() {
                events.push(Action::Options(OptionsAction::SortBy(col.clone())));
            }
        });

        if shortcut_button(ui, t!("Refresh"), shortcuts::REFRESH).clicked() {
            events.push(Action::Files(FileAction::Refresh));
        }
    });

    events
}
