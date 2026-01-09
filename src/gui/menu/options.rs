use egui::{Id, RichText, ScrollArea, Ui, style::ScrollStyle};

use crate::gui::{
    actions::Action,
    data::{
        constants::labels,
        state::{OpenWindow, State},
        table_columns::TableColumns,
    },
    utils::{check_button, open_external_window, pre_render},
};

pub fn menu(
    ui: &mut Ui, columns: &[(TableColumns, bool)], always_on_top: bool, mark_same: bool, state: &State,
) -> Vec<Action> {
    let mut events = Vec::new();

    ui.menu_button(labels::OPTIONS_MENU, |ui| {
        if ui.button(labels::CHOOSE_COLUMNS).clicked() {
            events.push(Action::OpenExternalWindow(
                OpenWindow::ChooseColunms,
                pre_render(ui.ctx(), |ui| {
                    choose_columns(ui, columns);
                }),
            ));
        }

        if check_button(ui, labels::HIGHLIGHT, mark_same).clicked() {
            events.push(Action::ToggleMarkSame);
        }

        if check_button(ui, labels::ALWAYS_ON_TOP, always_on_top).clicked() {
            events.push(Action::ToggleAlwaysOnTop);
            if *state.always_on_top() {
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::Normal));
            } else {
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
            }
        }

        egui::widgets::global_theme_preference_buttons(ui);
    });

    if state.open_extra_window == OpenWindow::ChooseColunms {
        events.extend(open_external_window(
            ui,
            OpenWindow::ChooseColunms,
            state.extra_window_size.unwrap(),
            &|ui| Some(choose_columns(ui, columns)),
        ));
    }

    events
}

fn choose_columns(ui: &mut Ui, columns: &[(TableColumns, bool)]) -> Vec<Action> {
    let mut events = Vec::new();

    ui.spacing_mut().scroll = ScrollStyle::solid();
    ScrollArea::vertical().show(ui, |ui| {
        let response = egui_dnd::dnd(ui, labels::CHOOSE_COLUMNS).show_custom(|ui, item_iter| {
            for (idx, (column, is_checked)) in columns.iter().enumerate() {
                item_iter.next(ui, Id::new(column), idx, true, |ui, item_handle| {
                    item_handle.ui(ui, |ui, handle, _state| {
                        handle.ui(ui, |ui| {
                            let mut check = *is_checked;
                            let response = ui.checkbox(&mut check, RichText::new(column.to_string()).heading());
                            if response.changed() {
                                events.push(Action::ToggleColumn(column.clone()));
                            }
                        });
                    })
                });
            }
        });

        if let Some(update) = response.final_update() {
            events.push(Action::DragColumn(update));
        }
    });

    events
}
