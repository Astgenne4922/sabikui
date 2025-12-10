use crate::gui::{
    actions::{Action, ActionHandler},
    body::Body,
    data::{constants::shortcuts, state::State},
    menu::Menu,
};
use eframe::{App, CreationContext, NativeOptions, run_native};
use egui::{Button, Event, KeyboardShortcut, ModifierNames, Response};
use std::sync::mpsc;

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
            let mut events = Vec::new();

            if !i.raw.dropped_files.is_empty() {
                events.push(Action::AddFolders(Some(
                    i.raw
                        .dropped_files
                        .iter()
                        .map(|f| f.path.clone().unwrap())
                        .collect::<Vec<_>>(),
                )));
            }

            if i.consume_shortcut(&shortcuts::SELECT_ALL) {
                events.push(Action::SelectAll);
            }
            if i.events.iter().any(|ev| matches!(ev, egui::Event::Copy)) {
                events.push(Action::CopySelected);
            }
            if i.consume_shortcut(&shortcuts::DESELECT_ALL) {
                events.push(Action::DeselectAll);
            }
            if i.consume_shortcut(&shortcuts::SAVE_SELECTED) {
                events.push(Action::SaveSelected);
            }
            for event in &i.events {
                if let Event::Paste(to_paste) = event {
                    events.push(Action::Paste(to_paste.clone()));
                }
            }
            if i.events.iter().any(|ev| matches!(ev, egui::Event::Cut)) {
                events.push(Action::ClearAll);
            }
            if i.consume_shortcut(&shortcuts::ADD_FILE) {
                events.push(Action::AddFiles(None));
            }
            if i.consume_shortcut(&shortcuts::ADD_FOLDER) {
                events.push(Action::AddFolders(None));
            }
            if i.consume_shortcut(&shortcuts::ADD_WILDCARD) {
                events.push(Action::AddWildcard);
            }
            if i.consume_shortcut(&shortcuts::REFRESH) {
                events.push(Action::Refresh);
            }
            if i.consume_shortcut(&shortcuts::CLEAR_SELECTED) {
                events.push(Action::ClearSelected);
            }

            for event in events {
                self.message_sender
                    .send(event)
                    .expect("The receiver should always be available");
            }
        });

        self.action_handler.handle(&mut self.state);
        if let Some(to_copy) = self.state.to_copy.take() {
            ctx.copy_text(to_copy);
        }
    }
}

fn shortcut_button(ui: &mut egui::Ui, label: &str, shortcut: KeyboardShortcut) -> Response {
    ui.add(Button::new(label).shortcut_text(shortcut.format(&ModifierNames::NAMES, cfg!(target_os = "macos"))))
}
