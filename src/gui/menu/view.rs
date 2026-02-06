use egui::Ui;

use crate::gui::{
    actions::{Action, files::FileAction, options::OptionsAction},
    constants::{labels, shortcuts},
    data::table_columns::TableColumns,
    utils::{long_submenu, shortcut_button, sort_button},
};

pub fn menu(
    ui: &mut Ui, active_columns: &[TableColumns], sorting_column: Option<&(TableColumns, bool)>,
) -> Vec<Action> {
    let mut events = Vec::new();

    ui.menu_button(labels::VIEW_MENU, |ui| {
        long_submenu(ui, labels::SORT_BY, active_columns, |ui, col| {
            if sort_button(ui, col, sorting_column).clicked() {
                events.push(Action::Options(OptionsAction::SortBy(col.clone())));
            }
        });

        if shortcut_button(ui, labels::REFRESH, shortcuts::REFRESH).clicked() {
            events.push(Action::Files(FileAction::Refresh));
        }
    });

    events
}
