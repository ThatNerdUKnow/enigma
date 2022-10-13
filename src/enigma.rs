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

#[cfg(test)]
mod tests {
    use crate::{
        common::Character,
        plugboard::{Plugboard, Plugs},
        reflector::Reflectors,
        rotor::{RotorConfig, Rotors},
    };

    use super::Enigma;

    #[test]
    fn codec() {
        let e = {
            let rotors: RotorConfig =
                RotorConfig::try_from([(Rotors::I, 'A'), (Rotors::II, 'X'), (Rotors::IV, 'N')])
                    .unwrap();
            let plugs = Plugs::try_from(vec![]).unwrap();
            let plugboard: Plugboard = Plugboard::try_from(plugs).unwrap();
            let reflector: Reflectors = Reflectors::B;

            Enigma::new(rotors, plugboard, reflector)
        };
        (0..1000).into_iter().for_each(|n| {
            ('A'..='Z')
                .into_iter()
                .map(|c| Character::try_from(c).unwrap())
                .for_each(|c| {
                    let ct = e.encode_at(c, n);
                    let pt = e.encode_at(ct, n);
                    assert_eq!(pt, c)
                })
        })
    }
}
