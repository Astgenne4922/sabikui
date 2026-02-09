use egui::{Event, Ui};

use crate::gui::{
    actions::{Action, files::FileAction, options::OptionsAction, selection::SelectionAction},
    constants::shortcuts,
    data::table_columns::TableColumns,
    utils::{long_submenu, shortcut_button},
};
use rust_i18n::t;

pub fn menu(ui: &mut Ui, active_columns: &[TableColumns]) -> Vec<Action> {
    let mut events = Vec::new();

    ui.menu_button(t!("Edit"), |ui| {
        if shortcut_button(ui, t!("Copy Selected"), shortcuts::COPY_SELECTED).clicked() {
            events.push(Action::Selection(SelectionAction::CopySelected));
        }

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

        long_submenu(ui, t!("Copy"), active_columns, |ui, col| {
            if ui.button(col.to_string()).clicked() {
                events.push(Action::Options(OptionsAction::CopyProperty(col.clone())));
            }
        });

        ui.separator();

        if shortcut_button(ui, t!("Select All"), shortcuts::SELECT_ALL).clicked() {
            events.push(Action::Selection(SelectionAction::SelectAll));
        }

        if shortcut_button(ui, t!("Deselect All"), shortcuts::DESELECT_ALL).clicked() {
            events.push(Action::Selection(SelectionAction::DeselectAll));
        }
    });

    events
}
