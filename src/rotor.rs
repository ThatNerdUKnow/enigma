use std::{hash::Hash, str::FromStr};

use crate::{
    cipher::{Cipher, Decode, Encode},
    common::Position,
};
use bruh_moment::Bruh;
use strum_macros::EnumString;

/// This enum represents each available rotor for the real life enigma machine
/// Each rotor is a simple substition cipher plus one or two notches which would allow the next rotor in the sequence to rotate
#[derive(EnumString, Hash, PartialEq, Eq, Clone)]
pub enum Rotors {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
}
/// Individual rotor used in the rotor mechanism
pub struct Rotor {
    position: Position,
    inital_pos: Position,
    cipher: Cipher,
    notches: Notches,
}

impl TryFrom<(Rotors, char)> for Rotor {
    type Error = Bruh;

    fn try_from((variant, position): (Rotors, char)) -> Result<Self, Self::Error> {
        match variant {
            Rotors::I => Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", &['Q'], position),
            Rotors::II => Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", &['E'], position),
            Rotors::III => Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", &['V'], position),
            Rotors::IV => Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", &['J'], position),
            Rotors::V => Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", &['Z'], position),
            Rotors::VI => Rotor::new("JPGVOUMFYQBENHZRDKASXLICTW", &['Z', 'M'], position),
            Rotors::VII => Rotor::new("NZJHGRCXMYSWBOUFAIVLPEKQDT", &['Z', 'M'], position),
            Rotors::VIII => Rotor::new("FKQHTLXOCBJSPDZRAMEWNIUYGV", &['Z', 'M'], position),
        }
    }
}

impl Rotor {
    fn new(c: &str, n: &[char], p: char) -> Result<Rotor, Bruh> {
        let cipher = Cipher::from_str(c).unwrap();
        let notches = n.iter().map(|p| Position::try_from(*p).unwrap()).collect();
        let position = Position::try_from(p)?;
        Ok(Rotor {
            position: position,
            cipher: cipher,
            notches: notches,
            inital_pos: position,
        })
    }

    fn get_notches(self) -> Notches {
        self.notches
    }
}

#[derive(Hash)]
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
    use crate::{
        cipher::{Decode, Encode},
        common::Character,
    };

    use super::{Rotor, Rotors};

    #[test]
    fn construct_i() {
        let _ = Rotor::try_from((Rotors::I, 'A'));
    }

    #[test]
    fn construct_ii() {
        let _ = Rotor::try_from((Rotors::II, 'A'));
    }

    #[test]
    fn construct_iii() {
        let _ = Rotor::try_from((Rotors::III, 'A'));
    }

    #[test]
    fn construct_iv() {
        let _ = Rotor::try_from((Rotors::IV, 'A'));
    }

    #[test]
    fn construct_v() {
        let _ = Rotor::try_from((Rotors::V, 'A'));
    }

    #[test]
    fn construct_vi() {
        let _ = Rotor::try_from((Rotors::VI, 'A'));
    }

    #[test]
    fn construct_vii() {
        let _ = Rotor::try_from((Rotors::VII, 'A'));
    }

    #[test]
    fn construct_viii() {
        let _ = Rotor::try_from((Rotors::VIII, 'A'));
    }

    #[test]
    fn construct_all_positions() {
        ('A'..='Z').into_iter().for_each(|c| {
            let _ = Rotor::try_from((Rotors::I, c));
        })
    }

    #[test]
    fn codec() {
        let r = Rotor::try_from((Rotors::I, 'A')).unwrap();
        let plaintext = Character::try_from('A').unwrap();
        let ciphertext = r.encode(plaintext);
        let res = r.decode(ciphertext);

        println!("plaintext {}", plaintext);
        println!("ciphertext {}", ciphertext);
        println!("res {}", res);

        assert_eq!(plaintext, res);
    }
}
