use crate::gui::{
    actions::{Action, ActionHandler},
    body::Body,
    data::{hashed_file::HashedFile, state::State},
    menu::Menu,
};
use eframe::{App, CreationContext, NativeOptions, run_native};
use egui::{Key, KeyboardShortcut, Modifiers};
use std::{path::Path, sync::mpsc};

mod actions;
mod body;
mod data;
mod menu;

pub fn run() {
    let native_options = NativeOptions::default();
    run_native("Sabikui", native_options, Box::new(|cc| Ok(Box::new(Sabikui::new(cc))))).unwrap();
}

struct Sabikui {
    state: State,
    menu: Menu,
    body: Body,
    action_handler: ActionHandler,
    message_sender: mpsc::Sender<Action>,
}

impl Sabikui {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "jet_brains_mono_nerd".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                "../assets/JetBrainsMonoNerdFontMono-Regular.ttf"
            ))),
        );

        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "jet_brains_mono_nerd".to_owned());

        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .push("jet_brains_mono_nerd".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        let (sx, rx) = mpsc::channel();

        Self {
            state: State::new(),
            menu: Menu::new(sx.clone()),
            body: Body::new(sx.clone()),
            action_handler: ActionHandler::new(rx),
            message_sender: sx,
        }
    }
}

impl App for Sabikui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.menu.show(ctx, &mut self.state);
        self.body.show(ctx, &mut self.state);

        ctx.input_mut(|i| {
            if !i.raw.dropped_files.is_empty() {
                for file in &i.raw.dropped_files {
                    let path = file.path.as_ref().unwrap();
                    self.hash_path(path);
                }
            }

            if i.consume_shortcut(&KeyboardShortcut::new(Modifiers::COMMAND, Key::A)) {
                self.message_sender.send(Action::SelectAll).unwrap();
            }
            if i.consume_shortcut(&KeyboardShortcut::new(Modifiers::COMMAND, Key::C)) {
                self.message_sender.send(Action::ClearSelected).unwrap();
            }
            if i.consume_shortcut(&KeyboardShortcut::new(Modifiers::COMMAND, Key::D)) {
                self.message_sender.send(Action::DeselectAll).unwrap();
            }
            if i.consume_shortcut(&KeyboardShortcut::new(Modifiers::COMMAND, Key::S)) {
                self.message_sender.send(Action::SaveSelected).unwrap();
            }
            if i.consume_shortcut(&KeyboardShortcut::new(Modifiers::COMMAND, Key::V)) {
                self.message_sender.send(Action::Paste).unwrap();
            }
            if i.events.iter().any(|ev| matches!(ev, egui::Event::Cut)) {
                self.message_sender.send(Action::ClearAll).unwrap();
            }
            if i.consume_shortcut(&KeyboardShortcut::new(Modifiers::NONE, Key::F2)) {
                self.message_sender.send(Action::AddFile).unwrap();
            }
            if i.consume_shortcut(&KeyboardShortcut::new(Modifiers::NONE, Key::F3)) {
                self.message_sender.send(Action::AddFolder).unwrap();
            }
            if i.consume_shortcut(&KeyboardShortcut::new(Modifiers::NONE, Key::F4)) {
                self.message_sender.send(Action::AddWildcard).unwrap();
            }
            if i.consume_shortcut(&KeyboardShortcut::new(Modifiers::NONE, Key::F5)) {
                self.message_sender.send(Action::Refresh).unwrap();
            }
            if i.consume_shortcut(&KeyboardShortcut::new(Modifiers::NONE, Key::Delete)) {
                self.message_sender.send(Action::ClearSelected).unwrap();
            }
        });

        self.action_handler.handle(&mut self.state);
    }
}

impl Sabikui {
    fn hash_path(&mut self, path: &Path) {
        if path.is_dir() {
            for entry in path.read_dir().unwrap().flatten() {
                self.hash_path(&entry.path());
            }
        } else {
            if self.state.files.iter().any(|f| f.path == *path) {
                return;
            }

            self.state
                .files
                .push(HashedFile::new(path, &self.state.algorithm_list()));
        }
    }
}
