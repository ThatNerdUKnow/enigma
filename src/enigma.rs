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

    pub fn encode_at(&self, c: Character, n: usize) -> Character {
        let plugboard_enc = self.plugboard.encode(c);
        let rotor_enc = self.rotors.encode_at(plugboard_enc, n);
        let reflector_enc = self.reflector.encode(rotor_enc);
        let rotor_dec = self.rotors.decode_at(reflector_enc, n);
        let plugboard_dec = self.plugboard.decode(rotor_dec);
        plugboard_dec
    }
}
