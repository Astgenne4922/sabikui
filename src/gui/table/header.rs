use egui::{Button, Color32, RichText, Stroke, Ui};

use crate::gui::{actions::Action, constants::icons, data::table_columns::TableColumns};

pub fn button(ui: &mut Ui, column: &TableColumns, sorting_column: Option<&(TableColumns, bool)>) -> Option<Action> {
    let label = if let Some((col, is_reversed)) = sorting_column
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

    let style = ui.style_mut();
    style.visuals.widgets.active.bg_stroke = Stroke::NONE;
    style.visuals.widgets.open.bg_stroke = Stroke::NONE;
    style.visuals.widgets.hovered.bg_stroke = Stroke::NONE;
    style.visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    style.visuals.widgets.inactive.bg_stroke = Stroke::NONE;

    ui.add_sized(
        ui.available_size(),
        Button::new(RichText::new(column.to_string()).strong())
            .corner_radius(0.0)
            .right_text(label),
    )
    .clicked()
    .then_some(Action::SortBy(column.clone()))
}
