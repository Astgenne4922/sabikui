use egui::{Align, CentralPanel, Event, Layout, Response, ScrollArea, Sense, TextStyle};
use egui_extras::{Column, TableBuilder};
use std::sync::mpsc;

use crate::gui::{
    actions::Action,
    data::{state::State, table_columns::TableColumns},
};

pub struct Body {
    message_sender: mpsc::Sender<Action>,
}

impl Body {
    pub fn new(message_sender: mpsc::Sender<Action>) -> Self {
        Self { message_sender }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut State) {
        CentralPanel::default().show(ctx, |ui| {
            let response = ui.interact(ui.min_rect(), ui.unique_id(), Sense::click());
            self.context_menu(response, state);

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

    fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        let text_height = TextStyle::Body
            .resolve(ui.style())
            .size
            .max(ui.spacing().interact_size.y);
        let available_height = ui.available_height();

        let columns = state
            .columns
            .iter()
            .filter_map(|(col, is_active)| if *is_active { Some(col.clone()) } else { None })
            .collect::<Vec<_>>();

        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(Layout::left_to_right(Align::Min))
            .columns(
                Column::remainder().at_least(100.0).clip(true).resizable(true),
                columns.len() + 1,
            )
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height)
            .sense(Sense::click())
            .header(20.0, |mut header| {
                for col in &columns {
                    header.col(|ui| {
                        ui.style_mut().interaction.selectable_labels = false;
                        ui.strong(col.to_string());
                    });
                }
            })
            .body(|body| {
                body.rows(text_height, state.files.len(), |mut row| {
                    let file = &state.files[row.index()];

                    row.set_selected(state.selected_rows.contains(&row.index()));

                    for col in &columns {
                        row.col(|ui| {
                            ui.style_mut().interaction.selectable_labels = false;
                            let text = match col {
                                TableColumns::Path => file.path.display().to_string(),
                                TableColumns::FileName => file.file_name(),
                                TableColumns::Algorithms(alg) => file.get_digest(&alg).unwrap().to_owned(),
                                TableColumns::LastEdit => file.last_edit(),
                                TableColumns::FileSize => file.size().to_string(),
                                TableColumns::Extension => file.extension(),
                            };
                            ui.label(format!("{}", text));
                        });
                    }

                    self.context_menu(row.response(), state);

                    if row.response().clicked() {
                        let index = row.index();
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

    fn context_menu(&self, response: Response, state: &mut State) {
        response.context_menu(|ui| {
            ui.add_enabled_ui(!state.selected_rows.is_empty(), |ui| {
                if ui.button("Save Selected     CTRL + S").clicked() {
                    self.message_sender.send(Action::SaveSelected).unwrap();
                }

                if ui.button("Copy Selected     CTRL + C").clicked() {
                    self.message_sender.send(Action::CopySelected).unwrap();
                }

                ui.menu_button("Copy", |ui| {
                    for alg in &state.algorithm_list() {
                        if ui.button(alg.to_ascii_uppercase()).clicked() {
                            self.message_sender.send(Action::CopyHash(alg.to_owned())).unwrap();
                        }
                    }
                });
            });

            ui.separator();

            if ui.button("Explorer Paste    CTRL + V").clicked() {
                ui.ctx().input(|i| {
                    for event in &i.events {
                        if let Event::Paste(to_paste) = event {
                            self.message_sender.send(Action::Paste(to_paste.clone())).unwrap();
                        }
                    }
                });
            }

            ui.separator();

            if ui.button("Refresh                 F5").clicked() {
                self.message_sender.send(Action::Refresh).unwrap();
            }
        });
    }
}
