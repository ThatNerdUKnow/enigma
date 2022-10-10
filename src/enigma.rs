use anyhow::{anyhow, Error};
use itertools::Itertools;

use crate::{
    cipher::{Decode, Encode},
    common::Character,
    plugboard::Plugboard,
    reflector::{Reflector, Reflectors},
    rotor::{Rotor, RotorConfig, Rotors},
};

struct Enigma {
    rotors: RotorConfig,
    plugboard: Plugboard,
    reflector: Reflector,
}

impl Enigma {
    pub fn new(rotors: RotorConfig, plugboard: Plugboard, reflector: Reflectors) -> Enigma {
        let reflector = Reflector::from(reflector);

        Enigma {
            rotors: rotors,
            plugboard: plugboard,
            reflector: reflector,
        }
    }
}
