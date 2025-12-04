use egui::{Align, CentralPanel, Layout, ScrollArea, Sense, TextStyle};
use egui_extras::{Column, TableBuilder};
use std::collections::HashSet;

use crate::gui::data::hashed_file::HashedFile;

/*
TODO Columns
    [ ] Filename
    [ ] [ALG]
    [ ] Last Edit
    [ ] File Size
    [ ] Extension

TODO Context menu
    [ ] Save selected - CTRL+S
    [ ] Copy selected - CTRL+C
    [ ] Explorer paste - CTRL+V

    [ ] Copy [ALG]

    [ ] Refresh - F5
*/

#[derive(Default)]
pub struct Body {
    selected_rows: HashSet<usize>,
    last_selected: usize,
}

impl Body {
    pub fn show(&mut self, ctx: &egui::Context, algorithms: &[String], files: &[HashedFile]) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::horizontal()
                .stick_to_bottom(true)
                .auto_shrink(false)
                .show(ui, |ui| {
                    self.ui(ui, algorithms, files);
                });
        });
    }

    fn ui(&mut self, ui: &mut egui::Ui, algorithms: &[String], files: &[HashedFile]) {
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
                algorithms.len() + 1,
            )
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height)
            .sense(Sense::click())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.style_mut().interaction.selectable_labels = false;
                    ui.strong("File Name");
                });
                for alg in algorithms {
                    header.col(|ui| {
                        ui.style_mut().interaction.selectable_labels = false;
                        ui.strong(alg);
                    });
                }
            })
            .body(|body| {
                body.rows(text_height, files.len(), |mut row| {
                    let file = &files[row.index()];

                    row.set_selected(self.selected_rows.contains(&row.index()));

                    row.col(|ui| {
                        ui.style_mut().interaction.selectable_labels = false;
                        ui.label(format!("{}", file.path.display()));
                    });

                    for alg in algorithms {
                        row.col(|ui| {
                            ui.style_mut().interaction.selectable_labels = false;
                            ui.label(file.get_digest(alg).unwrap());
                        });
                    }

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
}
