use std::fs;

use clap::Parser;
use inquire::Text;
use interface::{Args, Config};
use libenigma::enigma::Enigma;

mod interface;
fn main() {
    let args = Args::parse();

    let config: Config = match args.config {
        Some(path) => match fs::read_to_string(&path) {
            Ok(content) => serde_yaml::from_str(&content).unwrap(),
            Err(_) => {
                let config = Config::new();
                let cfg_string = serde_yaml::to_string(&config).unwrap();
                fs::write(path, cfg_string).unwrap();
                config
            }
        },
        None => Config::new(),
    };

    let e = Enigma::try_from(config).unwrap();

    let plaintext: String = match args.input {
        Some(path) => {
            let buf = fs::read(path).unwrap();
            String::from_utf8(buf).unwrap()
        }
        None => Text::new("Please enter text to encode:").prompt().unwrap(),
    };

    let ciphertext = e.encode(&plaintext);

    match args.output {
        Some(path) => fs::write(path, ciphertext).unwrap(),
        None => println!("{ciphertext}"),
    }
}
