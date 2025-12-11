use egui_extras::TableRow;

use crate::gui::{
    actions::Action,
    data::{state::State, table_columns::TableColumns},
    table::context_menu,
};

// TODO move logic to action module with event
pub fn rows(row: &mut TableRow, columns: &[TableColumns], state: &mut State) -> Vec<Action> {
    let mut events = Vec::new();

    let index = row.index();
    let file = &state.files[index];

    row.set_selected(state.selected_rows.contains(&index));

    for col in columns {
        row.col(|ui| {
            ui.style_mut().interaction.selectable_labels = false;
            ui.label(file.get_from_column(col));
        });
    }

    events.extend(context_menu::open(
        &row.response(),
        &state.algorithm_list(),
        !state.selected_rows.is_empty(),
    ));

    if row.response().clicked() {
        // SHIFT LEFT CLICK
        if row.response().ctx.input(|i| i.modifiers.shift_only()) {
            let range = if index < state.last_selected {
                index..=state.last_selected
            } else {
                state.last_selected..=index
            };
            state.selected_rows.clear();
            for i in range {
                state.selected_rows.insert(i);
            }
        } else {
            // LEFT CLICK
            if !row.response().ctx.input(|i| i.modifiers.command_only()) {
                state.selected_rows.clear();
                state.last_selected = 0;
            } // else CTRL LEFT CLICK
            if state.selected_rows.contains(&index) {
                state.selected_rows.remove(&index);
            } else {
                state.selected_rows.insert(index);
                state.last_selected = index;
            }
        }
    }

    events
}
