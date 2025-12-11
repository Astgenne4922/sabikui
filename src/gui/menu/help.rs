use egui::Ui;

use crate::gui::data::constants::labels;

pub fn menu(ui: &mut Ui) {
    ui.menu_button(labels::HELP_MENU, |ui| {
        if ui.button(labels::ABOUT).clicked() {
            // TODO About
        }

        if ui.button(labels::GITHUB).clicked() {
            ui.ctx()
                .open_url(egui::OpenUrl::new_tab("https://github.com/Astgenne4922/sabikui"));
        }
    });
}
