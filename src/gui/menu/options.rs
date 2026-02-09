use egui::{
    Id, RichText, ScrollArea, Ui, ViewportCommand, WindowLevel, scroll_area::ScrollSource, style::ScrollStyle, widgets,
};

use crate::gui::{
    actions::{Action, files::FileAction, options::OptionsAction, selection::SelectionAction, window::WindowAction},
    data::{
        state::{AsyncAction, OpenWindow, State},
        table_columns::TableColumns,
    },
    utils::{check_button, open_window},
};
use rust_i18n::t;

pub fn menu(
    ui: &mut Ui, columns: &[(TableColumns, bool)], always_on_top: bool, mark_same: bool, state: &State,
) -> Vec<Action> {
    let mut events = Vec::new();

    ui.menu_button(t!("Options"), |ui| {
        if ui.button(t!("Choose Columns")).clicked() {
            events.push(Action::Window(WindowAction::OpenExternalWindow(
                OpenWindow::ChooseColunms,
            )));
        }

        if check_button(ui, t!("Highlight identical Hashes"), mark_same).clicked() {
            events.push(Action::Options(OptionsAction::ToggleMarkSame));
        }

        if check_button(ui, t!("Always on Top"), always_on_top).clicked() {
            events.push(Action::Options(OptionsAction::ToggleAlwaysOnTop));
            if *state.always_on_top() {
                ui.ctx()
                    .send_viewport_cmd(ViewportCommand::WindowLevel(WindowLevel::Normal));
            } else {
                ui.ctx()
                    .send_viewport_cmd(ViewportCommand::WindowLevel(WindowLevel::AlwaysOnTop));
            }
        }

        widgets::global_theme_preference_buttons(ui);
    });

    if state.open_extra_window == OpenWindow::ChooseColunms {
        events.extend(open_window(ui.ctx(), t!("Choose Columns"), true, |ui| {
            if state.async_action == AsyncAction::HashProcessing {
                ui.disable();
            }
            Some(choose_columns(ui, columns))
        }));
    }

    events
}

fn choose_columns(ui: &mut Ui, columns: &[(TableColumns, bool)]) -> Vec<Action> {
    let mut events = Vec::new();

    ui.spacing_mut().scroll = ScrollStyle::solid();
    ScrollArea::vertical()
        .scroll_source(ScrollSource {
            scroll_bar: true,
            drag: false,
            mouse_wheel: true,
        })
        .show(ui, |ui| {
            let response = egui_dnd::dnd(ui, t!("Choose Columns")).show_custom(|ui, item_iter| {
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
                                    events.push(Action::Files(FileAction::ToggleColumn(column.clone())));
                                }
                            });
                        })
                    });
                }
            });

            if let Some(update) = response.final_update() {
                events.push(Action::Selection(SelectionAction::DragColumn(update)));
            }
        });

    events
}
