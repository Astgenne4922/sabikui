use egui::{CentralPanel, ScrollArea, Sense, TextStyle};
use egui_extras::{Column, TableBuilder};
use std::sync::mpsc;

use crate::gui::{actions::Action, data::state::State};

mod body;
mod context_menu;
mod header;

pub struct Body {
    message_sender: mpsc::Sender<Action>,
}

impl Body {
    pub const fn new(message_sender: mpsc::Sender<Action>) -> Self {
        Self { message_sender }
    }

    pub fn show(&self, ctx: &egui::Context, state: &mut State) {
        let mut events = Vec::new();

        CentralPanel::default().show(ctx, |ui| {
            events.extend(context_menu::open(
                &ui.interact(ui.min_rect(), ui.unique_id(), Sense::click()),
                &state.algorithm_list(),
                !state.selected_rows.is_empty(),
            ));

            ScrollArea::horizontal()
                .stick_to_bottom(true)
                .auto_shrink(false)
                .scroll_source(egui::scroll_area::ScrollSource {
                    scroll_bar: true,
                    drag: false,
                    mouse_wheel: true,
                })
                .show(ui, |ui| {
                    let text_height = TextStyle::Body
                        .resolve(ui.style())
                        .size
                        .max(ui.spacing().interact_size.y);
                    let available_height = ui.available_height();

                    let columns = state.active_columns();

                    TableBuilder::new(ui)
                        .striped(true)
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
                                    if let Some(event) = header::button(ui, column, state.sorting_column.as_ref()) {
                                        events.push(event);
                                    }
                                });
                            }
                        })
                        .body(|body| {
                            body.rows(text_height, state.files.len(), |mut row| {
                                events.extend(body::rows(&mut row, &columns, state));
                            });
                        });
                });
        });

        for event in events {
            self.message_sender
                .send(event)
                .expect("The receiver should always be available");
        }
    }
}
