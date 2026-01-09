use egui::{Button, KeyboardShortcut, ModifierNames, Response, Ui};

use crate::gui::{
    actions::Action,
    data::{constants::icons, table_columns::TableColumns},
};

pub fn shortcut_button(ui: &mut Ui, label: &str, shortcut: KeyboardShortcut) -> Response {
    ui.add(Button::new(label).shortcut_text(shortcut.format(&ModifierNames::NAMES, cfg!(target_os = "macos"))))
}

pub fn check_button(ui: &mut Ui, label: &str, is_checked: bool) -> Response {
    let check = if is_checked { "󰄬" } else { "" };
    ui.add(Button::new(label).right_text(check))
}

pub fn sort_button(ui: &mut Ui, column: &TableColumns, sorting_column: Option<&(TableColumns, bool)>) -> Response {
    let symbol = if let Some((col, is_reversed)) = sorting_column
        && col == column
    {
        if *is_reversed {
            icons::DESCENDING
        } else {
            icons::ASCENDING
        }
    } else {
        ""
    };
    ui.add(
        Button::new(column.to_string())
            .wrap_mode(egui::TextWrapMode::Extend)
            .right_text(symbol),
    )
}

pub fn open_window(
    ctx: &egui::Context, title: String, add_content: impl FnOnce(&mut Ui) -> Option<Vec<Action>>,
) -> Vec<Action> {
    let mut events = Vec::new();

    let mut open = true;
    egui::Window::new(title)
        .collapsible(false)
        .resizable(false)
        .open(&mut open)
        .show(ctx, |ui| {
            if let Some(output) = add_content(ui) {
                events.extend(output);
            }
        });
    if !open {
        events.push(Action::CloseExternalWindow);
    }

    events
}
