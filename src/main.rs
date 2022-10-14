/*mod cipher;
mod common;
mod enigma;
mod plugboard;
mod reflector;
mod rotor;*/

use enigma::cipher::{Cipher, Decode};
use enigma::common::Character;
use std::str::FromStr;

fn main() {
    ('A'..='z')
        .into_iter()
        .for_each(|c| match Character::try_from(c) {
            Ok(output) => println!("{}:{:?}", c, output),
            Err(message) => println!("{},{}", c, message),
        });

    let cipher = Cipher::from_str("ZYXWVUTSRQPONMLKJIHGFEDCBA").unwrap();

    let c = Character::try_from('A').unwrap();

    let _x = cipher.decode(c);
}
