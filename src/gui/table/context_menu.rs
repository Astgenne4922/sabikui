use egui::{Event, Response};
use rust_i18n::t;

use crate::gui::{
    actions::{Action, files::FileAction, options::OptionsAction, selection::SelectionAction},
    constants::shortcuts,
    data::table_columns::TableColumns,
    utils::{long_submenu, shortcut_button},
};

pub fn open(response: &Response, are_row_selected: bool, active_columns: &[TableColumns]) -> Vec<Action> {
    let mut events = Vec::new();

    response.context_menu(|ui| {
        ui.add_enabled_ui(are_row_selected, |ui| {
            if shortcut_button(ui, t!("Save Selected"), shortcuts::SAVE_SELECTED).clicked() {
                events.push(Action::Selection(SelectionAction::SaveSelected));
            }

            if shortcut_button(ui, t!("Copy Selected"), shortcuts::COPY_SELECTED).clicked() {
                events.push(Action::Selection(SelectionAction::CopySelected));
            }

            long_submenu(ui, t!("Copy"), active_columns, |ui, col| {
                if ui.button(col.to_string()).clicked() {
                    events.push(Action::Options(OptionsAction::CopyProperty(col.clone())));
                }
            });
        });

        ui.separator();

        if shortcut_button(ui, t!("Explorer Paste"), shortcuts::EXPLORER_PASTE).clicked() {
            ui.ctx().input(|i| {
                for event in &i.events {
                    if let Event::Paste(to_paste) = event {
                        events.push(Action::Files(FileAction::Paste(to_paste.clone())));
                    }
                }
            });
        }

        ui.separator();

        if shortcut_button(ui, t!("Refresh"), shortcuts::REFRESH).clicked() {
            events.push(Action::Files(FileAction::Refresh));
        }
    });

    events
}
