use egui::{Id, RichText, ScrollArea, Ui, style::ScrollStyle};

use crate::gui::{
    actions::Action,
    data::{
        constants::labels,
        state::{OpenWindow, State},
        table_columns::TableColumns,
    },
    utils::{check_button, open_window},
};

pub fn menu(
    ui: &mut Ui, columns: &[(TableColumns, bool)], always_on_top: bool, mark_same: bool, state: &State,
) -> Vec<Action> {
    let mut events = Vec::new();

    ui.menu_button(labels::OPTIONS_MENU, |ui| {
        if ui.button(labels::CHOOSE_COLUMNS).clicked() {
            events.push(Action::OpenExternalWindow(OpenWindow::ChooseColunms));
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
        events.extend(open_window(ui.ctx(), labels::CHOOSE_COLUMNS.to_string(), |ui| {
            Some(choose_columns(ui, columns))
        }));
    }

    events
}

fn choose_columns(ui: &mut Ui, columns: &[(TableColumns, bool)]) -> Vec<Action> {
    let mut events = Vec::new();

    ui.spacing_mut().scroll = ScrollStyle::solid();
    ScrollArea::vertical()
        .scroll_source(egui::scroll_area::ScrollSource {
            scroll_bar: true,
            drag: false,
            mouse_wheel: true,
        })
        .show(ui, |ui| {
            let response = egui_dnd::dnd(ui, labels::CHOOSE_COLUMNS).show_custom(|ui, item_iter| {
                for (idx, (column, is_checked)) in columns.iter().enumerate() {
                    item_iter.next(ui, Id::new(column), idx, true, |ui, item_handle| {
                        item_handle.ui(ui, |ui, handle, _state| {
                            handle.ui(ui, |ui| {
                                if matches!(column, TableColumns::FileName) {
                                    ui.disable();
                                }
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
