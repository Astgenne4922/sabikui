use sha3::{Digest as _, Sha3_256};
use std::{fs, io};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    for file in &args[1..] {
        let mut file = fs::File::open(file).unwrap();
        let mut hasher = Sha3_256::new();
        io::copy(&mut file, &mut hasher).unwrap();

        println!("{}", base16ct::lower::encode_string(&hasher.finalize()));
    }
}
