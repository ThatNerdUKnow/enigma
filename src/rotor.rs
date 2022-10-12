use std::{hash::Hash, str::FromStr};

use crate::{
    cipher::{Cipher, Decode, Encode},
    common::{Character, Position},
};
use anyhow::anyhow;
use bruh_moment::Bruh;
use itertools::Itertools;
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
    cipher: Cipher,
    notches: Notches,
}

pub struct RotorConfig(Vec<Rotor>);

impl TryFrom<[(Rotors, char); 3]> for RotorConfig {
    type Error = Bruh;

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
        })
    }

    fn get_notches(self) -> Notches {
        self.notches
    }

    fn encode_at(&self, c: Character, n: usize) -> Character {
        let offset: Position = self.position + n;
        self.cipher.encode(c + offset)
    }

    fn decode_at(&self, c: Character, n: usize) -> Character {
        let offset: Position = self.position + n;
        self.cipher.decode(c - offset)
    }

    /// given n revolutions of the current rotor, how many times will the next rotor in the sequence advance?
    fn get_num_advances(&self, n: usize) -> usize {
        let r = n / 26;
        let notches_left = self
            .notches
            .0
            .iter()
            .filter(|notch| self.position <= **notch)
            .count();

        let final_position = Position::try_from((n % 26) as u8).unwrap();
        let notches_past = self
            .notches
            .0
            .iter()
            .filter(|notch| final_position > **notch)
            .count();

        (r * self.notches.0.len()) + notches_left + notches_past
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

#[cfg(test)]
mod tests {
    use crate::{
        cipher::{Decode, Encode},
        common::{Character, Position},
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
        let r = Rotor::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ", &['A'], 'A').unwrap();

        (0..=u8::MAX).into_iter().for_each(|n| {
            ('A'..='Z')
                .into_iter()
                .map(|c| Character::try_from(c).unwrap())
                .for_each(|plaintext| {
                    let ciphertext = r.encode_at(plaintext, n.into());
                    let res = r.decode_at(ciphertext, n.into());

                    println!("{n}: {plaintext}-{ciphertext}-{res}");
                    // fix subtraction code

                    assert_eq!(plaintext, res);
                });
        })
    }
}
