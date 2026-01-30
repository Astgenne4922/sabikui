pub mod files;
pub mod options;
pub mod selection;
pub mod window;

use std::sync::mpsc::{Receiver, Sender};

use crate::gui::data::state::State;

#[derive(Clone)]
pub enum Action {
    Files(files::FileAction),
    Selection(selection::SelectionAction),
    Options(options::OptionsAction),
    Window(window::WindowAction),
}

pub struct ActionHandler {
    message_receiver: Receiver<Action>,
}

impl ActionHandler {
    pub const fn new(message_receiver: Receiver<Action>) -> Self {
        Self { message_receiver }
    }

    pub fn handle(&self, state: &mut State, sender: &Sender<Action>) {
        while let Ok(action) = self.message_receiver.try_recv() {
            match action {
                Action::Files(action) => files::handle(state, &action, sender),
                Action::Selection(action) => selection::handle(state, &action),
                Action::Options(action) => options::handle(state, &action),
                Action::Window(action) => window::handle(state, &action),
            }
        }
    }
}
