use crate::gui::{body::Body, data::hashed_file::HashedFile, menu::Menu};
use eframe::{App, CreationContext, NativeOptions, run_native};
use std::path::Path;

mod body;
mod data;
mod menu;

pub fn run() {
    let native_options = NativeOptions::default();
    run_native("Sabikui", native_options, Box::new(|cc| Ok(Box::new(Sabikui::new(cc))))).unwrap();
}

struct Sabikui {
    files: Vec<HashedFile>,
    menu: Menu,
    body: Body,
}

impl Sabikui {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            files: Vec::default(),
            menu: Menu::default(),
            body: Body::default(),
        }
    }
}

impl App for Sabikui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.menu.show(ctx, &mut self.files);
        let active_algorithms = self.menu.algorithm_list();

        self.body.show(ctx, &active_algorithms, &self.files);

        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                for file in &i.raw.dropped_files {
                    let path = file.path.as_ref().unwrap();
                    self.hash_path(&active_algorithms, path);
                }
            }
        });
    }
}

impl Sabikui {
    fn hash_path(&mut self, algorithms: &[String], path: &Path) {
        if path.is_dir() {
            for entry in path.read_dir().unwrap().flatten() {
                self.hash_path(algorithms, &entry.path());
            }
        } else {
            if self.files.iter().any(|f| f.path == *path) {
                return;
            }

            self.files.push(HashedFile::new(path, algorithms));
        }
    }
}
