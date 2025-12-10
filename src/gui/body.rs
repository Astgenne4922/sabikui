use egui::{Button, CentralPanel, Color32, Event, Response, RichText, ScrollArea, Sense, Stroke, TextStyle};
use egui_extras::{Column, TableBuilder};
use std::sync::mpsc;

use crate::gui::{
    actions::Action,
    data::{
        constants::{icons, labels, shortcuts},
        state::State,
        table_columns::TableColumns,
    },
    shortcut_button,
};

pub struct Body {
    message_sender: mpsc::Sender<Action>,
}

impl Body {
    pub const fn new(message_sender: mpsc::Sender<Action>) -> Self {
        Self { message_sender }
    }

    pub fn show(&self, ctx: &egui::Context, state: &mut State) {
        CentralPanel::default().show(ctx, |ui| {
            let response = ui.interact(ui.min_rect(), ui.unique_id(), Sense::click());
            self.context_menu(&response, state);

            ScrollArea::horizontal()
                .stick_to_bottom(true)
                .auto_shrink(false)
                .scroll_source(egui::scroll_area::ScrollSource {
                    scroll_bar: true,
                    drag: false,
                    mouse_wheel: true,
                })
                .show(ui, |ui| {
                    self.ui(ui, state);
                });
        });
    }

    fn ui(&self, ui: &mut egui::Ui, state: &mut State) {
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
                        self.header_button(ui, state, column);
                    });
                }
            })
            .body(|body| {
                body.rows(text_height, state.files.len(), |mut row| {
                    let index = row.index();
                    let file = &state.files[index];

                    row.set_selected(state.selected_rows.contains(&index));

                    for col in &columns {
                        row.col(|ui| {
                            ui.style_mut().interaction.selectable_labels = false;
                            ui.label(file.get_from_column(col));
                        });
                    }

                    self.context_menu(&row.response(), state);

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
                });
            });
    }

    fn header_button(&self, ui: &mut egui::Ui, state: &State, column: &TableColumns) {
        let label = if let Some((col, is_reversed)) = &state.sorting_column
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

        if ui
            .add_sized(
                ui.available_size(),
                Button::new(RichText::new(column.to_string()).strong())
                    .corner_radius(0.0)
                    .right_text(label),
            )
            .clicked()
        {
            self.message_sender
                .send(Action::SortBy(column.clone()))
                .expect("The receiver should always be available");
        }
    }

    fn context_menu(&self, response: &Response, state: &State) {
        response.context_menu(|ui| {
            let mut events = Vec::new();

            ui.add_enabled_ui(!state.selected_rows.is_empty(), |ui| {
                if shortcut_button(ui, labels::SAVE_SELECTED, shortcuts::SAVE_SELECTED).clicked() {
                    events.push(Action::SaveSelected);
                }

                if shortcut_button(ui, labels::COPY_SELECTED, shortcuts::COPY_SELECTED).clicked() {
                    events.push(Action::CopySelected);
                }

                ui.menu_button(labels::COPY, |ui| {
                    for alg in &state.algorithm_list() {
                        if ui.button(alg.to_ascii_uppercase()).clicked() {
                            events.push(Action::CopyHash(alg.to_owned()));
                        }
                    }
                });
            });

            ui.separator();

            if shortcut_button(ui, labels::EXPLORER_PASTE, shortcuts::EXPLORER_PASTE).clicked() {
                ui.ctx().input(|i| {
                    for event in &i.events {
                        if let Event::Paste(to_paste) = event {
                            events.push(Action::Paste(to_paste.clone()));
                        }
                    }
                });
            }

            ui.separator();

            if shortcut_button(ui, labels::REFRESH, shortcuts::REFRESH).clicked() {
                events.push(Action::Refresh);
            }

            for event in events {
                self.message_sender
                    .send(event)
                    .expect("The receiver should always be available");
            }
        });
    }
}
