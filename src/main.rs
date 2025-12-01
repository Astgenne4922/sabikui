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

    let mut algs = algorithms::get_hash_functions();

    match args.algorithms {
        Some(algorithms) => algs = algs.into_iter().filter(|a| algorithms.contains(a)).collect(),
        None => (),
    };

    if algs.len() == 0 {
        println!("Algorithm names not supported/recognized");

        return;
    }

    match args.input {
        Some(inputs) => {
            cli::run(inputs, &algs);
        }
        None => gui::run(),
    }
}
