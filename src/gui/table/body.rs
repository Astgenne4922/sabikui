use egui::Color32;
use egui_extras::TableRow;

use crate::gui::{
    actions::Action,
    data::{constants::colors, state::State, table_columns::TableColumns},
    table::context_menu,
};

pub fn rows(row: &mut TableRow, columns: &[TableColumns], state: &State) -> Vec<Action> {
    let mut events = Vec::new();

    let index = row.index();
    let file = &state.files()[index];

    row.set_selected(state.selected_rows().contains(&index));

    let mut color_idx = None;
    if *state.mark_same() {
        let same = state.same_hash_index();

        for (idx, pair) in same.iter().enumerate() {
            if *pair == file.get_digest(&state.algorithm_list()[0]) {
                color_idx = Some(idx % 14);
            }
        }
    }

    for col in columns {
        row.col(|ui| {
            ui.style_mut().interaction.selectable_labels = false;
            if let Some(color) = color_idx.map(|idx| {
                if ui.style().visuals.dark_mode {
                    colors::gruvbox::dark::HIGHLIGHT[idx]
                } else {
                    colors::gruvbox::light::HIGHLIGHT[idx]
                }
            }) && !state.selected_rows().contains(&index)
            {
                let gapless_rect = ui.max_rect().expand2(0.5 * ui.spacing().item_spacing);
                ui.painter().rect_filled(gapless_rect, 0.0, color);
                ui.style_mut().visuals.widgets.noninteractive.fg_stroke.color = Color32::from_rgb(40, 40, 40);
            }
            ui.label(file.get_from_column(col));
        });
    }

    events.extend(context_menu::open(
        &row.response(),
        !state.selected_rows().is_empty(),
        &state.active_columns(),
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
