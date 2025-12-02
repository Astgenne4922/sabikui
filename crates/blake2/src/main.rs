use blake2::{Blake2b512, Digest as _};
use std::{fs, io};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    for file in &args[1..] {
        let mut file = fs::File::open(file).unwrap();
        let mut hasher = Blake2b512::new();
        io::copy(&mut file, &mut hasher).unwrap();

        println!("{}", base16ct::lower::encode_string(&hasher.finalize()));
    }
}
