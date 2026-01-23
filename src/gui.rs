use crate::gui::{
    actions::{Action, ActionHandler},
    config::theme,
    constants::shortcuts,
    data::state::State,
    menu::Menu,
    table::Table,
};
use eframe::{App, CreationContext, NativeOptions, run_native};
use egui::Event;
use std::{fs, sync::mpsc};

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
    state: State,
    menu: Menu,
    table: Table,
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

        cc.egui_ctx.options_mut(|opt| {
            opt.dark_style = std::sync::Arc::new(egui::Style {
                visuals: theme::gruvbox!(dark),
                ..Default::default()
            });
            opt.light_style = std::sync::Arc::new(egui::Style {
                visuals: theme::gruvbox!(light),
                ..Default::default()
            });
        });

        let (sx, rx) = mpsc::channel();

        let state = fs::read(
            dirs::config_dir()
                .expect("[gui.rs - Sabikui::new]: There should be a config directory")
                .join(constants::CONFIG_DIRECTORY)
                .join(constants::CONFIG_FILE),
        )
        .ok()
        .and_then(|file| toml::from_slice(&file).ok())
        .unwrap_or_default();

        Self {
            state,
            menu: Menu::new(sx.clone()),
            table: Table::new(sx.clone()),
            action_handler: ActionHandler::new(rx),
            message_sender: sx,
        }
    }
}

impl Drop for Sabikui {
    fn drop(&mut self) {
        let config_folder = dirs::config_dir()
            .expect("[gui.rs - Sabikui::drop]: There should be a config directory")
            .join(constants::CONFIG_DIRECTORY);
        if !config_folder.exists() {
            fs::create_dir(&config_folder).unwrap(); // TODO handle errors
        }

        fs::write(
            config_folder.join(constants::CONFIG_FILE),
            toml::to_string_pretty(&self.state).expect("[gui.rs - Sabikui::drop]: The serialization should not fail"),
        )
        .unwrap(); // TODO handle errors
    }
}

impl App for Sabikui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.menu.show(ctx, &self.state);
        self.table.show(ctx, &self.state);

        ctx.input_mut(|i| {
            let mut events = Vec::new();

            if !i.raw.dropped_files.is_empty() {
                events.push(Action::AddFolders(Some(
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
                    Event::Copy => events.push(Action::CopySelected),
                    Event::Paste(to_paste) => events.push(Action::Paste(to_paste.clone())),
                    Event::Cut => events.push(Action::ClearAll),
                    _ => {}
                }
            }

            for event in events {
                self.message_sender
                    .send(event)
                    .expect("[gui.rs - Sabikui::update]: The receiver should always be available");
            }
        });

        self.action_handler.handle(&mut self.state);
        if let Some(to_copy) = self.state.to_copy.take() {
            ctx.copy_text(to_copy);
        }
    }
}
