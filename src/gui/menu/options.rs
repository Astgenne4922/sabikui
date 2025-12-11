use egui::Ui;

use crate::gui::{
    actions::Action,
    data::{constants::labels, state::State, table_columns::TableColumns},
};

// TODO move logic to action module with event
pub fn menu(ui: &mut Ui, state: &mut State) -> Vec<Action> {
    let mut events = Vec::new();

    ui.menu_button(labels::OPTIONS_MENU, |ui| {
        // TODO Options Columns
        ui.menu_button(labels::CHOOSE_COLUMNS, |ui| {
            for (column, is_checked) in &mut state.columns {
                let label = if *is_checked {
                    format!("󰄬  {column}")
                } else {
                    format!("   {column}")
                };
                if ui.button(label).clicked() {
                    *is_checked = !*is_checked;
                    if let TableColumns::Algorithms(alg) = column {
                        for file in &mut state.files {
                            if *is_checked {
                                file.add_digest_for(alg);
                            } else {
                                file.remove_digest(alg);
                            }
                        }
                    }
                }
            }
        });

        if ui.button(labels::HIGHLIGHT).clicked() {
            // TODO Mark identical hashes
        }

        ui.checkbox(&mut state.always_on_top, labels::ALWAYS_ON_TOP);
    });

    events
}
