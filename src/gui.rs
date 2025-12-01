use std::{collections::HashMap, path::PathBuf};

use eframe::{App, CreationContext, NativeOptions, run_native};
use egui::{Align, CentralPanel, Layout, MenuBar, ScrollArea, Sense, TextStyle, ahash::HashSet};
use egui_extras::{Column, TableBuilder};

use crate::algorithms::{self, many_hash_one_file, one_hash_one_file};

pub fn run() {
    let native_options = NativeOptions::default();
    run_native("Sabikui", native_options, Box::new(|cc| Ok(Box::new(Sabikui::new(cc))))).unwrap()
}

type HashedFile = (PathBuf, HashMap<String, String>);
struct Sabikui {
    algorithms: HashMap<String, bool>,
    files: Vec<HashedFile>,
    selected_rows: HashSet<usize>,
}

impl Default for Sabikui {
    fn default() -> Self {
        Self {
            algorithms: algorithms::get_hash_functions()
                .iter()
                .map(|h| (h.to_owned(), true))
                .collect::<HashMap<_, _>>(),
            files: Default::default(),
            selected_rows: Default::default(),
        }
    }
}

impl Sabikui {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl App for Sabikui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("Algorithms", |ui| {
                    for (alg, is_checked) in &mut self.algorithms {
                        if ui.checkbox(is_checked, alg.to_string()).changed() {
                            for (file, hashes) in self.files.iter_mut() {
                                if *is_checked {
                                    hashes.insert(alg.to_string(), one_hash_one_file(alg, &file));
                                } else {
                                    hashes.remove(alg);
                                }
                            }
                        }
                    }
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::horizontal()
                .stick_to_bottom(true)
                .auto_shrink(false)
                .show(ui, |ui| {
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
                            self.algorithm_list().len() + 1,
                        )
                        .min_scrolled_height(0.0)
                        .max_scroll_height(available_height)
                        .sense(Sense::click())
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.style_mut().interaction.selectable_labels = false;
                                ui.strong("File Name");
                            });
                            for alg in self.algorithm_list() {
                                header.col(|ui| {
                                    ui.style_mut().interaction.selectable_labels = false;
                                    ui.strong(format!("{}", alg));
                                });
                            }
                        })
                        .body(|body| {
                            body.rows(text_height, self.files.len(), |mut row| {
                                let (file, hashes) = &self.files[row.index()];

                                row.set_selected(self.selected_rows.contains(&row.index()));

                                row.col(|ui| {
                                    ui.style_mut().interaction.selectable_labels = false;
                                    ui.label(format!("{}", file.file_name().unwrap().display()));
                                });

                                for alg in self.algorithm_list() {
                                    row.col(|ui| {
                                        ui.style_mut().interaction.selectable_labels = false;
                                        ui.label(hashes.get(&alg).unwrap());
                                    });
                                }

                                if row.response().clicked() {
                                    if self.selected_rows.contains(&row.index()) {
                                        self.selected_rows.remove(&row.index());
                                    } else {
                                        self.selected_rows.insert(row.index());
                                    }
                                }
                            });
                        });
                });
        });

        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                for file in &i.raw.dropped_files {
                    let path = file.path.as_ref().unwrap();
                    self.hash_path(path);
                }
            }
        });
    }
}

impl Sabikui {
    fn algorithm_list(&self) -> Vec<String> {
        let mut algs = self
            .algorithms
            .iter()
            .filter_map(|(alg, is_checked)| if *is_checked { Some(alg.clone()) } else { None })
            .collect::<Vec<_>>();
        algs.sort();
        algs
    }

    fn hash_path(&mut self, path: &PathBuf) {
        if path.is_dir() {
            for entry in path.read_dir().unwrap() {
                if let Ok(entry) = entry {
                    self.hash_path(&entry.path());
                }
            }
        } else {
            if self.files.iter().any(|f| f.0 == *path) {
                return;
            }
            let digests = many_hash_one_file(&self.algorithm_list(), &path);

            self.files.push((
                path.clone(),
                self.algorithm_list()
                    .iter()
                    .map(|a| a.to_string())
                    .zip(digests)
                    .collect::<HashMap<_, _>>(),
            ));
        }
    }
}
