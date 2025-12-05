use egui::{Align, CentralPanel, Layout, Response, ScrollArea, Sense, TextStyle};
use egui_extras::{Column, TableBuilder};
use std::{collections::HashSet, sync::mpsc};

use crate::gui::{
    actions::Action,
    data::{hashed_file::HashedFile, table_columns::TableColumns},
};

pub struct Body {
    selected_rows: HashSet<usize>,
    last_selected: usize,
    message_sender: mpsc::Sender<Action>,
}

impl Body {
    pub fn new(message_sender: mpsc::Sender<Action>) -> Self {
        Self {
            selected_rows: Default::default(),
            last_selected: Default::default(),
            message_sender,
        }
    }
    pub fn show(&mut self, ctx: &egui::Context, algorithms: &[String], files: &[HashedFile], columns: &[TableColumns]) {
        CentralPanel::default().show(ctx, |ui| {
            let response = ui.interact(ui.min_rect(), ui.unique_id(), Sense::click());
            self.context_menu(response, algorithms, files, columns);

            ScrollArea::horizontal()
                .stick_to_bottom(true)
                .auto_shrink(false)
                .scroll_source(egui::scroll_area::ScrollSource {
                    scroll_bar: true,
                    drag: false,
                    mouse_wheel: true,
                })
                .show(ui, |ui| {
                    self.ui(ui, algorithms, files, columns);
                });
        });
    }

    fn ui(&mut self, ui: &mut egui::Ui, algorithms: &[String], files: &[HashedFile], columns: &[TableColumns]) {
        let text_height = TextStyle::Body
            .resolve(ui.style())
            .size
            .max(ui.spacing().interact_size.y);
        let available_height = ui.available_height();
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(Layout::left_to_right(Align::Min))
            .columns(
                Column::remainder().at_least(100.0).clip(true).resizable(true),
                // algorithms.len() + 1,
                columns.len() + 1,
            )
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height)
            .sense(Sense::click())
            .header(20.0, |mut header| {
                for col in columns {
                    header.col(|ui| {
                        ui.style_mut().interaction.selectable_labels = false;
                        ui.strong(col.to_string());
                    });
                }
                // header.col(|ui| {
                //     ui.style_mut().interaction.selectable_labels = false;
                //     ui.strong("File Name");
                // });
                // for alg in algorithms {
                //     header.col(|ui| {
                //         ui.style_mut().interaction.selectable_labels = false;
                //         ui.strong(alg);
                //     });
                // }
            })
            .body(|body| {
                body.rows(text_height, files.len(), |mut row| {
                    let file = &files[row.index()];

                    row.set_selected(self.selected_rows.contains(&row.index()));

                    for col in columns {
                        row.col(|ui| {
                            ui.style_mut().interaction.selectable_labels = false;
                            let text = match col {
                                TableColumns::Path => file.path.display().to_string(),
                                TableColumns::FileName => file.file_name(),
                                TableColumns::Algorithms(alg) => file.get_digest(&alg).unwrap().to_owned(),
                                TableColumns::LastEdit => format!("{:?}", file.last_edit()),
                                TableColumns::FileSize => file.size().to_string(),
                                TableColumns::Extension => file.extension(),
                            };
                            ui.label(format!("{}", text));
                        });
                    }
                    // row.col(|ui| {
                    //     ui.style_mut().interaction.selectable_labels = false;
                    //     ui.label(format!("{}", file.path.display()));
                    // });

                    // for alg in algorithms {
                    //     row.col(|ui| {
                    //         ui.style_mut().interaction.selectable_labels = false;
                    //         ui.label(file.get_digest(alg).unwrap());
                    //     });
                    // }

                    self.context_menu(row.response(), algorithms, files, columns);

                    if row.response().clicked() {
                        let index = row.index();
                        if row.response().ctx.input(|i| i.modifiers.shift_only()) {
                            let range = if index < self.last_selected {
                                index..=self.last_selected
                            } else {
                                self.last_selected..=index
                            };
                            self.selected_rows.clear();
                            for i in range {
                                self.selected_rows.insert(i);
                            }
                        } else {
                            if !row.response().ctx.input(|i| i.modifiers.command_only()) {
                                self.selected_rows.clear();
                                self.last_selected = 0;
                            }
                            if self.selected_rows.contains(&index) {
                                self.selected_rows.remove(&index);
                            } else {
                                self.selected_rows.insert(index);
                                self.last_selected = index;
                            }
                        }
                    }
                });
            });
    }

    fn context_menu(&self, response: Response, algorithms: &[String], files: &[HashedFile], columns: &[TableColumns]) {
        response.context_menu(|ui| {
            ui.add_enabled_ui(!self.selected_rows.is_empty(), |ui| {
                if ui.button("Save Selected     CTRL + S").clicked() {
                    self.message_sender.send(Action::SaveSelected).unwrap();
                }

                if ui.button("Copy Selected     CTRL + C").clicked() {
                    self.message_sender.send(Action::CopySelected).unwrap();
                }

                ui.menu_button("Copy", |ui| {
                    for alg in algorithms {
                        if ui.button(alg.to_ascii_uppercase()).clicked() {
                            self.message_sender.send(Action::CopyHash(alg.to_owned())).unwrap();
                        }
                    }
                });
            });

            ui.separator();

            if ui.button("Explorer Paste    CTRL + V").clicked() {
                self.message_sender.send(Action::Paste).unwrap();
            }

            ui.separator();

            if ui.button("Refresh                 F5").clicked() {
                self.message_sender.send(Action::Refresh).unwrap();
            }
        });
    }
}
