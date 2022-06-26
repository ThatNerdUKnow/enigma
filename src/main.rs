mod rotor;
mod reflector;
mod enigma;

use crate::enigma::{Enigma};
use crate::rotor::{rotors};
use crate::reflector::{reflectors};

fn main() {
    let message = "HELLOFROMENIGMA";
    let rotor_config = vec![rotors::I];

    let rotor_config_2 = rotor_config.clone();
    let mut enigma_machine = Enigma::new(rotor_config,reflectors::A);
    let mut enigma_2 = Enigma::new(rotor_config_2,reflectors::A);

    let cipher_text = enigma_machine.encode(message.to_string());
    println!("{}",cipher_text);

    let plain_text = enigma_2.encode(cipher_text);
    println!("{}",plain_text);
}
