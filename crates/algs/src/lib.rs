#[macro_export]
macro_rules! generate_main {
    ($hash:ident, $struct:ident) => {
        use $hash::{Digest as _, $struct};

        fn main() -> Result<(), Box<dyn std::error::Error>> {
            let input = std::env::args()
                .next()
                .ok_or("The program should take exactly one argument")?;

            for file in std::fs::read_to_string(input)?.lines() {
                let mut file = std::fs::File::open(file)?;
                let mut hasher = $struct::new();
                std::io::copy(&mut file, &mut hasher)?;

                println!("{}", base16ct::lower::encode_string(&hasher.finalize()));
            }

            Ok(())
        }
    };
}
