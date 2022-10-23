use clap::Parser;
use inquire::Text;
use interface::{Args, Config};
use libenigma::enigma::Enigma;

mod interface;
fn main() {
    let args = Args::parse();
    let config = Config::new();

    let e = Enigma::try_from(config).unwrap();

    let plaintext = Text::new("Please enter text to encode:").prompt().unwrap();

    let ciphertext = e.encode(&plaintext);

    println!("{ciphertext}")
}
