use egui::{CentralPanel, ScrollArea, Sense, TextStyle, style::ScrollStyle};
use egui_extras::{Column, TableBuilder};
use std::sync::mpsc;

use crate::gui::{
    actions::Action,
    data::state::{OpenWindow, State},
};

mod body;
mod context_menu;
mod drag_selector;
mod header;

pub struct Table {
    message_sender: mpsc::Sender<Action>,
}

impl Table {
    pub const fn new(message_sender: mpsc::Sender<Action>) -> Self {
        Self { message_sender }
    }

    pub fn show(&self, ctx: &egui::Context, state: &State) {
        let mut events = Vec::new();

        CentralPanel::default().show(ctx, |ui| {
            if *state.open_extra_window.read() != OpenWindow::None {
                ui.disable();
            }

            events.extend(context_menu::open(
                &ui.interact(ui.min_rect(), ui.unique_id(), Sense::click()),
                &state.algorithm_list(),
                !state.selected_rows().is_empty(),
            ));

            ui.interact(ui.min_rect(), ui.unique_id(), Sense::click_and_drag());
            let drag_response = ui.response();
            let mut painter = None;

            ui.spacing_mut().scroll = ScrollStyle::solid();
            let scroll_output = ScrollArea::horizontal()
                .stick_to_bottom(true)
                .auto_shrink(false)
                .scroll_source(egui::scroll_area::ScrollSource {
                    scroll_bar: true,
                    drag: false,
                    mouse_wheel: true,
                })
                .show(ui, |ui| {
                    if let Some(pointer_pos) = drag_response.interact_pointer_pos() {
                        let rect = egui::Rect::from_pos(pointer_pos);
                        ui.scroll_to_rect(rect, None);
                    }

                    let text_height = TextStyle::Body
                        .resolve(ui.style())
                        .size
                        .max(ui.spacing().interact_size.y);
                    let available_height = ui.available_height();

                    let columns = state.active_columns();

                    TableBuilder::new(ui)
                        .drag_to_scroll(false)
                        .resizable(true)
                        .columns(
                            Column::remainder().at_least(100.0).clip(true).resizable(true),
                            columns.len() + 1,
                        )
                        .min_scrolled_height(0.0)
                        .max_scroll_height(available_height)
                        .sense(Sense::click())
                        .header(20.0, |mut header| {
                            for column in &columns {
                                header.col(|ui| {
                                    if let Some(event) = header::button(ui, column, state.sorting_column()) {
                                        events.push(event);
                                    }
                                });
                            }
                        })
                        .body(|mut body| {
                            painter = Some(body.ui_mut().painter().clone());
                            if let Some(pointer_pos) = drag_response.interact_pointer_pos() {
                                let rect = egui::Rect::from_pos(pointer_pos);
                                body.ui_mut().scroll_to_rect(rect, None);
                            }

                            let mut visible_rows = Vec::new();

                            body.rows(text_height, state.files().len(), |mut row| {
                                events.extend(body::rows(&mut row, &columns, state));
                                visible_rows.push((row.response(), row.index()));
                            });

                            events.extend(drag_selector::handle(&visible_rows, &drag_response, state));
                        })
                        .state
                        .offset
                        .y
                });

            if ui.response().clicked() {
                events.push(Action::DeselectAll);
            }

            events.extend(drag_selector::show(
                &painter.expect("The painter should always be initialized by the table body"),
                &drag_response,
                state,
                scroll_output.state.offset.x,
                scroll_output.inner,
            ));
        });

        for event in events {
            self.message_sender
                .send(event)
                .expect("The receiver should always be available");
        }
    }
}
