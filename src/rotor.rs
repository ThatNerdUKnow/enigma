use std::str::FromStr;

use crate::{
    cipher::{Cipher, Decode, Encode},
    common::Position,
};
use strum_macros::EnumString;

#[derive(EnumString)]
enum Rotors {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
}
struct Rotor {
    position: Position,
    cipher: Cipher,
    notches: Notches,
}

impl Rotor {
    fn From(r: Rotors, p: char) -> Rotor {
        match r {
            Rotors::I => Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", &['Q'], p),
            Rotors::II => Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", &['E'], p),
            Rotors::III => Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", &['V'], p),
            Rotors::IV => Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", &['J'], p),
            Rotors::V => Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", &['Z'], p),
            Rotors::VI => Rotor::new("JPGVOUMFYQBENHZRDKASXLICTW", &['Z', 'M'], p),
            Rotors::VII => Rotor::new("NZJHGRCXMYSWBOUFAIVLPEKQDT", &['Z', 'M'], p),
            Rotors::VIII => Rotor::new("FKQHTLXOCBJSPDZRAMEWNIUYGV", &['Z', 'M'], p),
        }
    }

    fn new(c: &str, n: &[char], p: char) -> Rotor {
        let cipher = Cipher::from_str(c).unwrap();
        let notches = n.iter().map(|p| Position::try_from(*p).unwrap()).collect();
        let position = Position::try_from(p).unwrap();
        Rotor {
            position: position,
            cipher: cipher,
            notches: notches,
        }
    }
}

struct Notches(Vec<Position>);

impl FromIterator<Position> for Notches {
    fn from_iter<T: IntoIterator<Item = Position>>(iter: T) -> Self {
        let mut c = Notches(Vec::new());
        for i in iter {
            c.0.push(i)
        }
        c
    }
}

impl Encode for Rotor {
    fn encode(&self, c: crate::common::Character) -> crate::common::Character {
        let p = self.position;
        self.cipher.encode(c + p)
    }
}

impl Decode for Rotor {
    fn decode(&self, c: crate::common::Character) -> crate::common::Character {
        let p = self.position;
        self.cipher.decode(c + p)
    }
}

#[cfg(test)]
mod tests {
    use super::{Rotor, Rotors};

    #[test]
    fn construct_i() {
        Rotor::From(Rotors::I, 'A');
    }

    #[test]
    fn construct_ii() {
        Rotor::From(Rotors::II, 'A');
    }

    #[test]
    fn construct_iii() {
        Rotor::From(Rotors::III, 'A');
    }

    #[test]
    fn construct_iv() {
        Rotor::From(Rotors::IV, 'A');
    }

    #[test]
    fn construct_v() {
        Rotor::From(Rotors::V, 'A');
    }

    #[test]
    fn construct_vi() {
        Rotor::From(Rotors::VI, 'A');
    }

    #[test]
    fn construct_vii() {
        Rotor::From(Rotors::VII, 'A');
    }

    #[test]
    fn construct_viii() {
        Rotor::From(Rotors::VIII, 'A');
    }

    #[test]
    fn construct_all_positions() {
        ('A'..='Z').into_iter().for_each(|c| {
            Rotor::From(Rotors::I, c);
        })
    }
}
