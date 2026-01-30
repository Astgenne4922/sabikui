use egui::Ui;

use crate::gui::{
    actions::{Action, files::FileAction, options::OptionsAction},
    constants::{labels, shortcuts},
    data::{hashed_file::HashFunction, table_columns::TableColumns},
    utils::{long_submenu, shortcut_button, sort_button},
};

pub fn menu(
    ui: &mut Ui, algorithm_list: &[HashFunction], sorting_column: Option<&(TableColumns, bool)>,
) -> Vec<Action> {
    let mut events = Vec::new();

    ui.menu_button(labels::VIEW_MENU, |ui| {
        ui.menu_button(labels::SORT_BY, |ui| {
            if sort_button(ui, &TableColumns::FileName, sorting_column).clicked() {
                events.push(Action::Options(OptionsAction::SortBy(TableColumns::FileName)));
            }

            if sort_button(ui, &TableColumns::Path, sorting_column).clicked() {
                events.push(Action::Options(OptionsAction::SortBy(TableColumns::Path)));
            }

            long_submenu(ui, labels::ALGORITHM, algorithm_list, |ui, alg| {
                if sort_button(ui, &TableColumns::Algorithms(alg.clone()), sorting_column).clicked() {
                    events.push(Action::Options(OptionsAction::SortBy(TableColumns::Algorithms(
                        alg.clone(),
                    ))));
                }
            });

            if sort_button(ui, &TableColumns::LastEdit, sorting_column).clicked() {
                events.push(Action::Options(OptionsAction::SortBy(TableColumns::LastEdit)));
            }

            if sort_button(ui, &TableColumns::FileSize, sorting_column).clicked() {
                events.push(Action::Options(OptionsAction::SortBy(TableColumns::FileSize)));
            }

            if sort_button(ui, &TableColumns::Extension, sorting_column).clicked() {
                events.push(Action::Options(OptionsAction::SortBy(TableColumns::Extension)));
            }
        });

        if shortcut_button(ui, labels::REFRESH, shortcuts::REFRESH).clicked() {
            events.push(Action::Files(FileAction::Refresh));
        }
    });

    events
}
