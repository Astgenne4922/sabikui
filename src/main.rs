use crate::{algorithms::get_hash_functions, cli::parser};
use clap::Parser as _;

mod algorithms;
mod cli;

fn main() {
    let args = parser::Args::parse();

    if args.list {
        for hash in get_hash_functions() {
            println!("{hash}");
        }

        return;
    }

    match args.input {
        Some(inputs) => {
            cli::run(inputs);
        }
        None => todo!(), // gui::run()
    }
}
