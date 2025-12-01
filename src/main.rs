mod algorithms;
mod cli;
mod gui;

use cli::parser::Parser as _;

fn main() {
    let args = cli::parser::Args::parse();

    if args.list {
        for hash in algorithms::get_hash_functions() {
            println!("{hash}");
        }

        return;
    }

    match args.input {
        Some(inputs) => {
            cli::run(inputs);
        }
        None => gui::run(),
    }
}
