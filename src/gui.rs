use crate::gui::{
    actions::{Action, ActionHandler, files::FileAction, selection::SelectionAction},
    config::{
        state::{load_config, save_config},
        theme::{self, Theme, save_theme},
    },
    constants::{labels, shortcuts},
    data::state::{AsyncAction, OpenWindow, State},
    menu::Menu,
    table::Table,
};
use eframe::{App, CreationContext, Frame, NativeOptions, run_native};
use egui::{CentralPanel, Context, Event, FontData, FontDefinitions, FontFamily, TopBottomPanel, Ui};
use std::sync::{Arc, Mutex, mpsc};

mod actions;
mod config;
mod constants;
mod data;
mod menu;
mod table;
mod utils;

pub fn run() {
    let native_options = NativeOptions::default();
    run_native("Sabikui", native_options, Box::new(|cc| Ok(Box::new(Sabikui::new(cc))))).unwrap();
}

struct Sabikui {
    state: Arc<Mutex<State>>,
    dark_theme: Theme,
    light_theme: Theme,
    menu: Menu,
    table: Table,
    action_handler: ActionHandler,
    message_sender: mpsc::Sender<Action>,
}

impl Sabikui {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "jet_brains_mono_nerd".to_owned(),
            Arc::new(FontData::from_static(include_bytes!(
                "../assets/JetBrainsMonoNerdFontMono-Regular.ttf"
            ))),
        );

        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "jet_brains_mono_nerd".to_owned());

        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("jet_brains_mono_nerd".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        let dark_theme = theme::load_theme(theme::Mode::Dark).unwrap_or(theme::Theme::dark());
        let light_theme = theme::load_theme(theme::Mode::Light).unwrap_or(theme::Theme::light());
        cc.egui_ctx.options_mut(|opt| {
            opt.dark_style = Arc::new(egui::Style {
                visuals: dark_theme.visuals(),
                ..Default::default()
            });
            opt.light_style = Arc::new(egui::Style {
                visuals: light_theme.visuals(),
                ..Default::default()
            });
        });

        let (sx, rx) = mpsc::channel();

        Self {
            state: Arc::new(Mutex::new(load_config())),
            dark_theme,
            light_theme,
            menu: Menu::new(sx.clone()),
            table: Table::new(sx.clone()),
            action_handler: ActionHandler::new(rx),
            message_sender: sx,
        }
    }
}

impl Drop for Sabikui {
    fn drop(&mut self) {
        save_config(&self.state.lock().expect("The lock was poisoned"));

        save_theme(&self.dark_theme, theme::Mode::Dark);
        save_theme(&self.light_theme, theme::Mode::Light);
    }
}

impl App for Sabikui {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            let state = self.state.lock().expect("The lock was poisoned");
            disable_ui(ui, &state);
            self.menu.show(ui, &state);
        });

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            let state = self.state.lock().expect("The lock was poisoned");
            disable_ui(ui, &state);
            ui.style_mut().interaction.selectable_labels = false;
            ui.horizontal(|ui| {
                ui.label(format!("{} {}", state.files().len(), labels::BOTTOM_BAR_TEXT));

                if state.async_action == AsyncAction::HashProcessing {
                    ui.spinner();
                }
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            let state = self.state.lock().expect("The lock was poisoned");
            disable_ui(ui, &state);
            self.table.show(ui, &state);
        });

        ctx.input_mut(|i| {
            let mut events = Vec::new();

            if !i.raw.dropped_files.is_empty() {
                events.push(Action::Files(FileAction::AddFolders(
                    i.raw.dropped_files.iter().map(|f| f.path.clone().unwrap()).collect(),
                )));
            }

            for (shortcut, action) in shortcuts::KEYBINDS {
                if i.consume_shortcut(shortcut) {
                    events.push(action);
                }
            }

            for event in &i.events {
                match event {
                    Event::Copy => events.push(Action::Selection(SelectionAction::CopySelected)),
                    Event::Paste(to_paste) => events.push(Action::Files(FileAction::Paste(to_paste.clone()))),
                    Event::Cut => events.push(Action::Selection(SelectionAction::ClearAll)),
                    _ => {}
                }
            }

            for event in events {
                self.message_sender
                    .send(event)
                    .expect("The receiver should always be available");
            }
        });

        let to_copy = self.state.lock().expect("The lock was poisoned").to_copy.take();
        if let Some(to_copy) = to_copy {
            ctx.copy_text(to_copy);
        }

        self.action_handler.handle(&self.state, &self.message_sender);
    }
}

fn disable_ui(ui: &mut Ui, state: &State) {
    if state.open_extra_window != OpenWindow::None || state.async_action != AsyncAction::None {
        ui.disable();
    }
}
