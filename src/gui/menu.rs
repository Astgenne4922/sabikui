use crate::gui::{actions::Action, data::state::State};
use egui::{MenuBar, TopBottomPanel};
use std::sync::mpsc;

mod edit;
mod file;
mod help;
mod options;
mod tool;
mod view;

pub struct Menu {
    message_sender: mpsc::Sender<Action>,
}

impl Menu {
    pub const fn new(message_sender: mpsc::Sender<Action>) -> Self {
        Self { message_sender }
    }

    pub fn show(&self, ctx: &egui::Context, state: &State) {
        if *state.always_on_top() {
            ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::Normal));
        }

        let mut events = Vec::new();

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                MenuBar::new().ui(ui, |ui| {
                    events.extend(file::menu(ui));
                    events.extend(edit::menu(ui, &state.algorithm_list()));
                    events.extend(view::menu(ui, &state.algorithm_list(), state.sorting_column()));
                    events.extend(options::menu(
                        ui,
                        state.columns(),
                        *state.always_on_top(),
                        *state.mark_same(),
                    ));
                    help::menu(ui);
                });
                events.extend(tool::bar(ui));
            });
        });

        for event in events {
            self.message_sender
                .send(event)
                .expect("The receiver should always be available");
        }
    }
}
