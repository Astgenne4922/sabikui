use egui::{Color32, Frame, Id, ScrollArea, Ui, style::ScrollStyle};

use crate::gui::{
    actions::Action,
    data::{constants::labels, state::State, table_columns::TableColumns},
    utils::check_button,
};

pub fn menu(
    ui: &mut Ui,
    columns: &[(TableColumns, bool)],
    always_on_top: bool,
    mark_same: bool,
    state: &mut State,
) -> Vec<Action> {
    let mut events = Vec::new();

    ui.menu_button(labels::OPTIONS_MENU, |ui| {
        // TODO Options Columns
        ui.menu_button(labels::CHOOSE_COLUMNS, |ui| {
            ui.spacing_mut().scroll = ScrollStyle::solid();
            ScrollArea::vertical().show(ui, |ui| {
                let response = egui_dnd::dnd(ui, labels::CHOOSE_COLUMNS).show(
                    state.columns.iter(),
                    |ui, column, handle, _state| {
                        handle.ui(ui, |ui| {
                            if check_button(ui, &column.0.to_string(), column.1).clicked() {
                                events.push(Action::ToggleColumn(column.0.clone()));
                            }
                        });
                    },
                );
                response.update_vec(&mut state.columns);
            });
        });

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
