use anyhow::{anyhow, Error};
use itertools::Itertools;

use crate::{
    plugboard::Plugboard,
    reflector::{Reflector, Reflectors},
    rotor::{Rotor, Rotors},
};

struct Enigma {
    rotors: RotorConfig,
    plugboard: Plugboard,
    reflector: Reflector,
}

struct RotorConfig([Rotor; 3]);

impl TryFrom<[(Rotors, char); 3]> for RotorConfig {
    type Error = Error;

    fn try_from(value: [(Rotors, char); 3]) -> Result<Self, Self::Error> {
        let iter = value.iter();
        let rawcount = iter.clone().count();
        let unique = iter.clone().map(|(r, _)| r).unique().count();
        match (rawcount, unique) {
            (3, 3) => iter.cloned().map(|r| Rotor::try_from(r)).try_collect(),
            (_, 0..=2) => Err(anyhow!(
                "Duplicate rotors: You may not use the same rotor more than once"
            )),
            (0..=2, _) => Err(anyhow!("Too few rotors: you must provide 3 rotors")),
            _ => Err(anyhow!(
                "Too many rotors: you must provide exactly 3 rotors"
            )),
        }
    }
}

impl FromIterator<Rotor> for RotorConfig {
    fn from_iter<T: IntoIterator<Item = Rotor>>(iter: T) -> Self {
        let num = iter.into_iter().count();
        match num {
            0..=2 => panic!("Too few items in iterator to create rotorconfig"),
            3 => todo!(),
            _ => panic!("Too many items in iterator to create rotorconfig"),
        }
    }
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
