use sabikui::{algorithms, cli};

use cli::parser::Parser as _;

fn main() {
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

    if let Some(inputs) = args.input {
        if functions.is_empty() {
            println!("Algorithm names not supported/recognized");

            return;
        }
        cli::run(inputs, &functions);
    }
}
