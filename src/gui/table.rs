use egui::{CentralPanel, ScrollArea, Sense, TextStyle};
use egui_extras::{Column, TableBuilder};
use std::sync::mpsc;

use crate::gui::{actions::Action, data::state::State};

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
            events.extend(context_menu::open(
                &ui.interact(ui.min_rect(), ui.unique_id(), Sense::click()),
                &state.algorithm_list(),
                !state.selected_rows().is_empty(),
            ));
            let (mut painter, drag_response) = drag_selector::setup(ui);

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
                            // FIXME drag not working when not on rows
                            painter = body.ui_mut().painter().clone();
                            if let Some(pointer_pos) = drag_response.interact_pointer_pos() {
                                let rect = egui::Rect::from_pos(pointer_pos);
                                body.ui_mut().scroll_to_rect(rect, None);
                            }

                            let mut drag_end_index = None;

                            body.rows(text_height, state.files().len(), |mut row| {
                                events.extend(body::rows(&mut row, &columns, state));
                                if let Some(pointer_pos) = drag_response.interact_pointer_pos()
                                    && state.drag_start_index.is_some()
                                {
                                    let rect = egui::Rect::from_pos(pointer_pos);
                                    if rect.intersects(row.response().interact_rect) {
                                        drag_end_index = Some(row.index());
                                    }
                                }

                                if drag_response.drag_started() && row.response().contains_pointer() {
                                    events.push(Action::StartDragSelection(row.index()));
                                }
                            });

                            if let (Some(drag_start_index), Some(drag_end_index)) =
                                (state.drag_start_index, drag_end_index)
                            {
                                let range = if drag_start_index < drag_end_index {
                                    drag_start_index..=drag_end_index
                                } else {
                                    drag_end_index..=drag_start_index
                                };
                                events.push(Action::SelectRowRange(range));
                            }
                        })
                        .state
                        .offset
                        .y
                });

            events.extend(drag_selector::show(
                &painter,
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
