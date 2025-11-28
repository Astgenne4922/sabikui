use sha1::{Digest as _, Sha1};
use std::{fs, io};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    for file in &args[1..] {
        let mut file = fs::File::open(file).unwrap();
        let mut hasher = Sha1::new();
        let n = io::copy(&mut file, &mut hasher).unwrap();
        let hash = hasher.finalize();

        println!("{}", base16ct::lower::encode_string(&hash));
    }
}
