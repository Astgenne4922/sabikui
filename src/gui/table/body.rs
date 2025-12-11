use egui_extras::TableRow;

use crate::gui::{
    actions::Action,
    data::{state::State, table_columns::TableColumns},
    table::context_menu,
};

pub fn rows(row: &mut TableRow, columns: &[TableColumns], state: &State) -> Vec<Action> {
    let mut events = Vec::new();

    let index = row.index();
    let file = &state.files()[index];

    row.set_selected(state.selected_rows().contains(&index));

    for col in columns {
        row.col(|ui| {
            ui.style_mut().interaction.selectable_labels = false;
            ui.label(file.get_from_column(col));
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
