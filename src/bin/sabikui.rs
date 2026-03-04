use sabikui::{algorithms, cli};

use cli::parser::Parser as _;

fn main() {
    let args = cli::parser::Args::parse();

    if args.list {
        for hash in algorithms::get_hash_functions() {
            println!("{hash}");
        }
    } else {
        let mut functions = algorithms::get_hash_functions();
        if let Some(algorithms) = args.algorithms {
            functions.retain(|a| algorithms.contains(a));
        }

        if let Some(inputs) = args.input {
            if functions.is_empty() {
                eprintln!("Algorithm names not supported/recognized");
                std::process::exit(1);
            } else if let Err(err) = cli::run(inputs, &functions) {
                eprintln!("{err}");
                std::process::exit(1);
            }
        }
    }
}
