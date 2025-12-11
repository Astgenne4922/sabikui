use crate::gui::{
    actions::{Action, ActionHandler},
    data::{constants::shortcuts, state::State},
    menu::Menu,
    table::Body,
};
use eframe::{App, CreationContext, NativeOptions, run_native};
use egui::{Button, Event, KeyboardShortcut, ModifierNames, Response};
use std::sync::mpsc;

mod actions;
mod data;
mod menu;
mod table;

const KEYBINDS: [(&KeyboardShortcut, Action); 8] = [
    (&shortcuts::SELECT_ALL, Action::SelectAll),
    (&shortcuts::DESELECT_ALL, Action::DeselectAll),
    (&shortcuts::SAVE_SELECTED, Action::SaveSelected),
    (&shortcuts::ADD_FILE, Action::AddFiles(None)),
    (&shortcuts::ADD_FOLDER, Action::AddFolders(None)),
    (&shortcuts::ADD_WILDCARD, Action::AddWildcard),
    (&shortcuts::REFRESH, Action::Refresh),
    (&shortcuts::CLEAR_SELECTED, Action::ClearSelected),
];

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

            for (shortcut, action) in KEYBINDS {
                if i.consume_shortcut(shortcut) {
                    events.push(action);
                }
            }

            for event in &i.events {
                match event {
                    Event::Copy => events.push(Action::CopySelected),
                    Event::Paste(to_paste) => events.push(Action::Paste(to_paste.clone())),
                    Event::Cut => events.push(Action::ClearAll),
                    _ => {}
                }
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
