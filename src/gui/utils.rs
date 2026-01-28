use egui::{Button, KeyboardShortcut, ModifierNames, Response, ScrollArea, Ui, style::ScrollStyle};

use crate::gui::{
    actions::Action,
    constants::{icons, labels},
    data::table_columns::TableColumns,
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

pub fn long_submenu<T>(ui: &mut Ui, label: &str, items: &[T], mut add_item: impl FnMut(&mut Ui, &T)) {
    ui.scope(|ui| {
        if items.is_empty() {
            ui.disable();
        }
        ui.menu_button(label, |ui| {
            ui.spacing_mut().scroll = ScrollStyle::thin();
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
            ScrollArea::vertical()
                .scroll_source(egui::scroll_area::ScrollSource {
                    scroll_bar: true,
                    drag: false,
                    mouse_wheel: true,
                })
                .show(ui, |ui| {
                    for item in items {
                        add_item(ui, item);
                    }
                });
        });
    });
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

pub fn wildcard_window(ctx: &egui::Context) -> Vec<Action> {
    open_window(ctx, labels::ADD_WILDCARD.to_string(), |ui| {
        let mut events = Vec::new();
        let mut text: String = ctx.data(|data| data.get_temp("add_wildcard_text".into()).unwrap_or_default());

        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut text);
            if ui.button("...").clicked()
                && let Some(folder) = rfd::FileDialog::new().pick_folder()
            {
                text = folder.display().to_string();
            }
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui.button(labels::OK).clicked() {
                events.push(Action::AddWildcard(text.clone()));
            }

            if ui.button(labels::CANCEL).clicked() {
                events.push(Action::CloseExternalWindow);
            }
        });

        ui.add_space(5.0);

        ctx.data_mut(|data| {
            data.insert_temp("add_wildcard_text".into(), text);
        });

        Some(events)
    })
}
