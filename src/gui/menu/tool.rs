use egui::{FontId, Ui};

use crate::gui::{
    actions::Action,
    constants::{icons, labels},
    data::state::OpenWindow,
};

pub fn bar(ui: &mut Ui) -> Vec<Action> {
    let mut events = Vec::new();

    ui.horizontal(|ui| {
        ui.style_mut().override_font_id = Some(FontId::proportional(20.0));
        ui.spacing_mut().button_padding.x = 8.0;
        ui.spacing_mut().item_spacing.x = 1.0;

        if ui
            .button(icons::SAVE_SELECTED)
            .on_hover_text(labels::SAVE_SELECTED)
            .clicked()
        {
            events.push(Action::SaveSelected);
        }

        if ui.button(icons::REFRESH).on_hover_text(labels::REFRESH).clicked() {
            events.push(Action::Refresh);
        }

        ui.separator();

        if ui.button(icons::ADD_FILE).on_hover_text(labels::ADD_FILE).clicked() {
            events.push(Action::PickFiles);
        }

        if ui.button(icons::ADD_FOLDER).on_hover_text(labels::ADD_FOLDER).clicked() {
            events.push(Action::PickFolders);
        }

        if ui
            .button(icons::ADD_WILDCARD)
            .on_hover_text(labels::ADD_WILDCARD)
            .clicked()
        {
            events.push(Action::OpenExternalWindow(OpenWindow::AddWildcard));
        }

        ui.separator();

        if ui.button(icons::SELECT_ALL).on_hover_text(labels::SELECT_ALL).clicked() {
            events.push(Action::SelectAll);
        }

        if ui
            .button(icons::COPY_SELECTED)
            .on_hover_text(labels::COPY_SELECTED)
            .clicked()
        {
            events.push(Action::ClearSelected);
        }

        if ui.button(icons::CLEAR_ALL).on_hover_text(labels::CLEAR_ALL).clicked() {
            events.push(Action::ClearAll);
        }
    });

    events
}
