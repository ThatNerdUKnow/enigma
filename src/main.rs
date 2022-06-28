mod rotor;
mod reflector;
mod enigma;

use crate::enigma::{Enigma};
use crate::rotor::{RotorList, Rotor};
use crate::reflector::{ReflectorList, Reflector};

fn main() {
    let message = "HELLOFROMENIGMA";
    let rotor_config = vec![Rotor::from(RotorList::I,'A')];

    let rotor_config_2 = rotor_config.clone();
    let mut enigma_machine = Enigma::new(rotor_config,Reflector::from(ReflectorList::B));
    let mut enigma_2 = Enigma::new(rotor_config_2,Reflector::from(ReflectorList::B));

    let cipher_text = enigma_machine.encode(message.to_string());
    println!("{}",cipher_text);

    let plain_text = enigma_2.encode(cipher_text);
    println!("{}",plain_text);
}
