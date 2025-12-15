use egui::{Painter, Response, Sense, Ui, vec2};

use crate::gui::{actions::Action, data::state::State};

pub fn setup(ui: &Ui) -> (Painter, Response) {
    ui.interact(ui.min_rect(), ui.unique_id(), Sense::click_and_drag());
    let painter = ui.painter().clone();
    let response = ui.response();
    (painter, response)
}

pub fn show(painter: &Painter, response: &Response, state: &State, x_offset: f32, y_offset: f32) -> Vec<Action> {
    let mut events = Vec::new();
    if response.drag_started() {
        events.push(Action::StartDrag(
            response.interact_pointer_pos().unwrap(),
            vec2(x_offset, y_offset),
        ));
    }
    if response.drag_stopped() {
        events.push(Action::EndDrag);
    }

    if let (Some(pointer_pos), Some((drag_start, start_offset))) = (response.interact_pointer_pos(), state.drag_start) {
        let drag_start = drag_start + (start_offset - vec2(x_offset, y_offset));
        let min = pointer_pos.min(drag_start);
        let max = pointer_pos.max(drag_start);
        painter.rect(
            egui::Rect { min, max },
            0,
            egui::Color32::from_hex("#45858877").unwrap(),
            egui::Stroke {
                color: egui::Color32::from_hex("#458588").unwrap(),
                width: 1.0,
            },
            egui::StrokeKind::Middle,
        );
    }

    events
}
