use egui::{Painter, Response, Sense, Ui};

use crate::gui::{actions::Action, data::state::State};

pub fn setup(ui: &Ui) -> (Painter, Response) {
    ui.interact(ui.min_rect(), ui.unique_id(), Sense::click_and_drag());
    let painter = ui.painter().clone();
    let response = ui.response();
    (painter, response)
}

pub fn show(painter: &Painter, response: &Response, state: &State) -> Vec<Action> {
    let mut events = Vec::new();
    if response.drag_started() {
        events.push(Action::StartDrag(response.interact_pointer_pos().unwrap()));
    }
    if response.drag_stopped() {
        events.push(Action::EndDrag);
    }

    if let (Some(pointer_pos), Some(drag_start)) = (response.interact_pointer_pos(), state.drag_start) {
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
