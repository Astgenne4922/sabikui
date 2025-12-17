use egui::Ui;

use crate::gui::data::{
    constants::labels,
    state::{OpenWindow, State},
};

pub fn menu(ui: &mut Ui, state: &State) {
    ui.menu_button(labels::HELP_MENU, |ui| {
        if ui.button(labels::ABOUT).clicked() {
            // TODO About
            *state.open_extra_window.write() = OpenWindow::Help;
        }
        ui.hyperlink_to(labels::GITHUB, "https://github.com/Astgenne4922/sabikui");
    });

    if *state.open_extra_window.read() == OpenWindow::Help {
        let show_deferred_viewport = state.open_extra_window.clone();
        ui.ctx().show_viewport_deferred(
            egui::ViewportId::from_hash_of(OpenWindow::Help),
            egui::ViewportBuilder::default()
                .with_maximize_button(false)
                .with_minimize_button(false)
                .with_title("Sabikui")
                .with_resizable(false),
            move |ctx, class| {
                if class == egui::ViewportClass::Deferred {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Hello from deferred viewport");

                        if ui.input(|i| i.viewport().close_requested()) {
                            *show_deferred_viewport.write() = OpenWindow::None;
                        }
                    });
                }
            },
        );
    }
}
