use egui::{Button, KeyboardShortcut, ModifierNames, Response, Ui, Vec2};

use crate::gui::{
    actions::Action,
    data::{constants::icons, state::OpenWindow, table_columns::TableColumns},
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

pub fn pre_render(ctx: &egui::Context, add_content: impl FnOnce(&mut Ui)) -> egui::Vec2 {
    egui::Window::new("pre_render").title_bar(false).show(ctx, |ui| {
        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
        add_content(ui);
    });
    ctx.used_size()
}

pub fn open_external_window(
    ui: &Ui,
    window_type: OpenWindow,
    size: Vec2,
    add_content: &impl Fn(&mut Ui) -> Option<Vec<Action>>,
) -> Vec<Action> {
    let mut events = Vec::new();
    // println!("{size:?}");
    ui.ctx().show_viewport_immediate(
        egui::ViewportId::from_hash_of(window_type),
        egui::ViewportBuilder::default()
            .with_maximize_button(false)
            .with_minimize_button(false)
            .with_inner_size(size)
            .with_active(true)
            .with_taskbar(false)
            .with_title("Sabikui")
            .with_resizable(false),
        |ctx, class| {
            if class != egui::ViewportClass::Embedded {
                let inner_events = egui::CentralPanel::default()
                    .show(ctx, |ui| {
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                        add_content(ui)
                    })
                    .inner;
                if let Some(inner_events) = inner_events {
                    events.extend(inner_events);
                }
            }

            if ctx.input(|i| i.viewport().close_requested()) {
                events.push(Action::CloseExternalWindow);
            }
        },
    );

    events
}
