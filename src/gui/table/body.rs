use egui::Color32;
use egui_extras::TableRow;

use crate::gui::{
    actions::Action,
    data::{state::State, table_columns::TableColumns},
    table::context_menu,
};

pub fn rows(row: &mut TableRow, columns: &[TableColumns], state: &State) -> Vec<Action> {
    const COLORS: [Color32; 8] = [
        Color32::CYAN,
        Color32::GREEN,
        Color32::LIGHT_RED,
        Color32::BLUE,
        Color32::MAGENTA,
        Color32::YELLOW,
        Color32::PURPLE,
        Color32::ORANGE,
    ];
    let mut events = Vec::new();

    let index = row.index();
    let file = &state.files()[index];

    row.set_selected(state.selected_rows().contains(&index));

    let pairs = state.pair_same_hashes();

    let mut color = None;
    for (idx, pair) in pairs.iter().enumerate() {
        if pair.contains(&index) {
            color = Some(COLORS[idx % 8]);
        }
    }

    for col in columns {
        row.col(|ui| {
            ui.style_mut().interaction.selectable_labels = false;
            if let Some(color) = color {
                ui.colored_label(color, file.get_from_column(col));
            } else {
                ui.label(file.get_from_column(col));
            }
        });
    }

    events.extend(context_menu::open(
        &row.response(),
        &state.algorithm_list(),
        !state.selected_rows().is_empty(),
    ));

    if row.response().clicked() {
        if row.response().ctx.input(|i| i.modifiers.shift_only()) {
            // SHIFT LEFT CLICK
            let range = if index < *state.last_selected() {
                index..=*state.last_selected()
            } else {
                *state.last_selected()..=index
            };

            events.push(Action::SelectRowRange(range));
        } else if row.response().ctx.input(|i| i.modifiers.command_only()) {
            // CTRL LEFT CLICK
            events.push(Action::AddRowToSelection(index));
        } else {
            // LEFT CLICK
            events.push(Action::SelectSingleRow(index));
        }
    }

    events
}
