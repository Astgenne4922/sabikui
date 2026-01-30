use egui::{Color32, Painter, Rect, Response, Stroke, StrokeKind, vec2};

use crate::gui::{
    actions::{Action, selection::SelectionAction},
    data::state::State,
};

pub fn show(painter: &Painter, response: &Response, state: &State, x_offset: f32, y_offset: f32) -> Vec<Action> {
    let mut events = Vec::new();
    if response.drag_started() {
        events.push(Action::Selection(SelectionAction::StartDrag(
            response.interact_pointer_pos().unwrap(),
            vec2(x_offset, y_offset),
        )));
    }
    if response.drag_stopped() {
        events.push(Action::Selection(SelectionAction::EndDrag));
    }

    if let (Some(pointer_pos), Some((drag_start, start_offset))) = (response.interact_pointer_pos(), state.drag_start) {
        let drag_start = drag_start + (start_offset - vec2(x_offset, y_offset));
        let min = pointer_pos.min(drag_start);
        let max = pointer_pos.max(drag_start);
        painter.rect(
            Rect { min, max },
            0,
            Color32::from_hex("#45858877").unwrap(),
            Stroke {
                color: Color32::from_hex("#458588").unwrap(),
                width: 1.0,
            },
            StrokeKind::Middle,
        );
    }

    events
}

pub fn handle(visible_rows: &[(Response, usize)], drag_response: &Response, state: &State) -> Vec<Action> {
    let mut events = Vec::new();

    let mut drag_end_index = None;
    let mut first_visible_row = None;
    let mut last_visible_row = None;

    for (row_response, row_index) in visible_rows {
        if row_response.rect.is_finite() {
            if first_visible_row.is_none() {
                first_visible_row = Some((row_response.rect.min.y, row_index));
            }
            last_visible_row = Some((row_response.rect.max.y, row_index));
        }
        if let Some(pointer_pos) = drag_response.interact_pointer_pos()
            && state.drag_start_index.is_some()
            && row_response.interact_rect.y_range().contains(pointer_pos.y)
        {
            drag_end_index = Some(row_index);
        }

        if drag_response.drag_started() && row_response.contains_pointer() {
            events.push(Action::Selection(SelectionAction::SetDragSelectionIndex(*row_index)));
        }
    }

    // allows more precise selection when moving fast
    if let Some(pointer_pos) = drag_response.interact_pointer_pos()
        && state.drag_start_index.is_some()
    {
        if let Some((first_y, first_idx)) = first_visible_row
            && first_y > pointer_pos.y
        {
            drag_end_index = Some(first_idx);
        } else if let Some((last_y, last_idx)) = last_visible_row
            && last_y < pointer_pos.y
        {
            drag_end_index = Some(last_idx);
        }
    }

    // allows drag start on empty table body
    if let (Some(pointer_pos), Some((drag_start_pos, _))) = (drag_response.interact_pointer_pos(), state.drag_start) {
        if let Some((first_y, first_idx)) = first_visible_row
            && first_y > drag_start_pos.y
        {
            if pointer_pos.y > first_y {
                events.push(Action::Selection(SelectionAction::SetDragSelectionIndex(*first_idx)));
            } else {
                events.push(Action::Selection(SelectionAction::ResetDragSelectionIndex));
            }
        } else if let Some((last_y, last_idx)) = last_visible_row
            && last_y < drag_start_pos.y
        {
            if pointer_pos.y < last_y {
                events.push(Action::Selection(SelectionAction::SetDragSelectionIndex(*last_idx)));
            } else {
                events.push(Action::Selection(SelectionAction::ResetDragSelectionIndex));
            }
        }
    }

    if let (Some(drag_start_index), Some(drag_end_index)) = (state.drag_start_index, drag_end_index) {
        let range = if drag_start_index < *drag_end_index {
            drag_start_index..=*drag_end_index
        } else {
            *drag_end_index..=drag_start_index
        };
        events.push(Action::Selection(SelectionAction::SelectRowRange(range)));
    }

    events
}
