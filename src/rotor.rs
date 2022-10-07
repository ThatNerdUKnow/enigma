use std::{hash::Hash, str::FromStr};

use crate::{
    cipher::{Cipher, Decode, Encode},
    common::Position,
};
use anyhow::Error;
use strum_macros::EnumString;

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
pub struct Rotor {
    position: Position,
    inital_pos: Position,
    cipher: Cipher,
    notches: Notches,
}

impl Rotor {
    pub fn From(r: Rotors, p: char) -> Result<Rotor, Error> {
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

    fn new(c: &str, n: &[char], p: char) -> Result<Rotor, Error> {
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

    fn advance(&mut self) {
        self.position.advance()
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
        let _ = Rotor::From(Rotors::I, 'A');
    }

    #[test]
    fn construct_ii() {
        let _ = Rotor::From(Rotors::II, 'A');
    }

    #[test]
    fn construct_iii() {
        let _ = Rotor::From(Rotors::III, 'A');
    }

    #[test]
    fn construct_iv() {
        let _ = Rotor::From(Rotors::IV, 'A');
    }

    #[test]
    fn construct_v() {
        let _ = Rotor::From(Rotors::V, 'A');
    }

    #[test]
    fn construct_vi() {
        let _ = Rotor::From(Rotors::VI, 'A');
    }

    #[test]
    fn construct_vii() {
        let _ = Rotor::From(Rotors::VII, 'A');
    }

    #[test]
    fn construct_viii() {
        let _ = Rotor::From(Rotors::VIII, 'A');
    }

    #[test]
    fn construct_all_positions() {
        ('A'..='Z').into_iter().for_each(|c| {
            let _ = Rotor::From(Rotors::I, c);
        })
    }

    #[test]
    fn codec() {
        let r = Rotor::From(Rotors::I, 'A').unwrap();
        let plaintext = Character::try_from('A').unwrap();
        let ciphertext = r.encode(plaintext);
        let res = r.decode(ciphertext);

        println!("plaintext {}", plaintext);
        println!("ciphertext {}", ciphertext);
        println!("res {}", res);

        assert_eq!(plaintext, res);
    }
}
