#[macro_export]
macro_rules! generate_main {
    ($hash:ident, $struct:ident) => {
        use $hash::{Digest as _, $struct};

        fn main() {
            let args = std::env::args().collect::<Vec<_>>();

            for file in &args[1..] {
                let mut file = std::fs::File::open(file).expect("The file should exist and should be valid");
                let mut hasher = $struct::new();
                std::io::copy(&mut file, &mut hasher).expect("The file should be readable and the hasher should work");

                println!("{}", base16ct::lower::encode_string(&hasher.finalize()));
            }
        }
    };
}
