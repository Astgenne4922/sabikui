use egui::Ui;

use crate::gui::{
    actions::Action,
    data::{
        constants::{labels, shortcuts},
        hashed_file::HashFunction,
        table_columns::TableColumns,
    },
    utils::{shortcut_button, sort_button},
};

pub fn menu(
    ui: &mut Ui,
    algorithm_list: &[HashFunction],
    sorting_column: Option<&(TableColumns, bool)>,
) -> Vec<Action> {
    let mut events = Vec::new();

    ui.menu_button(labels::VIEW_MENU, |ui| {
        ui.menu_button(labels::SORT_BY, |ui| {
            if sort_button(ui, &TableColumns::FileName, sorting_column).clicked() {
                events.push(Action::SortBy(TableColumns::FileName));
            }

            if sort_button(ui, &TableColumns::Path, sorting_column).clicked() {
                events.push(Action::SortBy(TableColumns::Path));
            }

            ui.menu_button(labels::ALGORITHM, |ui| {
                for alg in algorithm_list {
                    if sort_button(ui, &TableColumns::Algorithms(alg.clone()), sorting_column).clicked() {
                        events.push(Action::SortBy(TableColumns::Algorithms(alg.clone())));
                    }
                }
            });

            if sort_button(ui, &TableColumns::LastEdit, sorting_column).clicked() {
                events.push(Action::SortBy(TableColumns::LastEdit));
            }

            if sort_button(ui, &TableColumns::FileSize, sorting_column).clicked() {
                events.push(Action::SortBy(TableColumns::FileSize));
            }

            if sort_button(ui, &TableColumns::Extension, sorting_column).clicked() {
                events.push(Action::SortBy(TableColumns::Extension));
            }
        });

        if shortcut_button(ui, labels::REFRESH, shortcuts::REFRESH).clicked() {
            events.push(Action::Refresh);
        }
    });

    events
}
