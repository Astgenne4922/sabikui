use egui::{Button, Context, KeyboardShortcut, ModifierNames, Response, Ui, Vec2, ViewportClass};

use crate::gui::data::{constants::icons, state::OpenWindow, table_columns::TableColumns};

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

pub fn open_external_window(
    ui: &Ui,
    window_type: OpenWindow,
    size: impl Into<Vec2>,
    viewport_ui_cb: impl Fn(&Context, ViewportClass) + Send + Sync + 'static,
) {
    ui.ctx().show_viewport_deferred(
        egui::ViewportId::from_hash_of(window_type),
        egui::ViewportBuilder::default()
            .with_maximize_button(false)
            .with_minimize_button(false)
            .with_inner_size(size)
            .with_active(true)
            .with_taskbar(false)
            .with_title("Sabikui")
            .with_resizable(false),
        viewport_ui_cb,
    );
}
