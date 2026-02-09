mod algorithms;
mod cli;
mod gui;

use cli::parser::Parser as _;

rust_i18n::i18n!();

fn main() {
    rust_i18n::set_locale(&sys_locale::get_locale().unwrap());

    let args = cli::parser::Args::parse();

    if args.list {
        for hash in algorithms::get_hash_functions() {
            println!("{hash}");
        }

        return;
    }

    let mut functions = algorithms::get_hash_functions();
    if let Some(algorithms) = args.algorithms {
        functions.retain(|a| algorithms.contains(a));
    }

    match args.input {
        Some(inputs) => {
            if functions.is_empty() {
                println!("Algorithm names not supported/recognized");

                return;
            }
            cli::run(inputs, &functions);
        }
        None => gui::run(),
    }
}
