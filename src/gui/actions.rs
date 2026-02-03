pub mod files;
pub mod options;
pub mod selection;
pub mod window;

use std::sync::{
    Arc, Mutex,
    mpsc::{Receiver, Sender},
};

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

    pub fn handle(&self, state: &Arc<Mutex<State>>, sender: &Sender<Action>) {
        while let Ok(action) = self.message_receiver.try_recv() {
            match action {
                Action::Files(action) => files::handle(Arc::clone(state), &action, sender),
                Action::Selection(action) => {
                    selection::handle(&mut state.lock().expect("The lock was poisoned"), &action);
                }
                Action::Options(action) => options::handle(&mut state.lock().expect("The lock was poisoned"), &action),
                Action::Window(action) => window::handle(&mut state.lock().expect("The lock was poisoned"), &action),
            }
        }
    }
}
