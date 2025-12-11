use egui::Ui;

use crate::gui::{
    actions::Action,
    data::{
        constants::{icons, labels, shortcuts},
        hashed_file::HashFunction,
        table_columns::TableColumns,
    },
    shortcut_button,
};

pub fn menu(
    ui: &mut Ui,
    algorithm_list: &[HashFunction],
    sorting_column: Option<&(TableColumns, bool)>,
) -> Vec<Action> {
    let mut events = Vec::new();

    ui.menu_button(labels::VIEW_MENU, |ui| {
        ui.menu_button(labels::SORT_BY, |ui| {
            if ui.button(make_label(&TableColumns::FileName, sorting_column)).clicked() {
                events.push(Action::SortBy(TableColumns::FileName));
            }

            if ui.button(make_label(&TableColumns::Path, sorting_column)).clicked() {
                events.push(Action::SortBy(TableColumns::Path));
            }

            ui.menu_button(labels::ALGORITHM, |ui| {
                for alg in algorithm_list {
                    if ui
                        .button(make_label(&TableColumns::Algorithms(alg.clone()), sorting_column))
                        .clicked()
                    {
                        events.push(Action::SortBy(TableColumns::Algorithms(alg.clone())));
                    }
                }
            });

            if ui.button(make_label(&TableColumns::LastEdit, sorting_column)).clicked() {
                events.push(Action::SortBy(TableColumns::LastEdit));
            }

            if ui.button(make_label(&TableColumns::FileSize, sorting_column)).clicked() {
                events.push(Action::SortBy(TableColumns::FileSize));
            }

            if ui
                .button(make_label(&TableColumns::Extension, sorting_column))
                .clicked()
            {
                events.push(Action::SortBy(TableColumns::Extension));
            }
        });

        if shortcut_button(ui, labels::REFRESH, shortcuts::REFRESH).clicked() {
            events.push(Action::Refresh);
        }
    });

    events
}

fn make_label(column: &TableColumns, sorting_column: Option<&(TableColumns, bool)>) -> String {
    if let Some((col, is_reversed)) = sorting_column
        && col == column
    {
        if *is_reversed {
            format!("{} {column}", icons::DESCENDING)
        } else {
            format!("{} {column}", icons::ASCENDING)
        }
    } else {
        format!("  {column}")
    }
}
