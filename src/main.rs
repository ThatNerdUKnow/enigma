mod rotor;
mod reflector;
mod enigma;

use std::env;

use crate::enigma::{Enigma};
use crate::rotor::{rotors};
use crate::reflector::{reflectors};

fn main() {
    env::set_var("RUST_BACKTRACE","1");
    let message = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let rotor_config = vec![rotors::I,rotors::II,rotors::III];

    let mut enigma_machine = Enigma::new(rotor_config,reflectors::A);

    let cipher_text = enigma_machine.encode(message.to_string());
    println!("{}",cipher_text);
}
