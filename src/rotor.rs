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
#[derive(EnumString, Hash, PartialEq, Eq, Clone, Copy)]
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

#[derive(Hash, Debug)]
struct Notches(Vec<Position>);

pub struct RotorConfig(Vec<Rotor>);

impl RotorConfig {
    pub fn encode_at(&self, c: Character, n: usize) -> Character {
        let encode_first_rotor = self.0[0].encode_at(c, n);
        let n = self.0[0].get_num_advances(n);
        let encode_second_rotor = self.0[1].encode_at(encode_first_rotor, n);
        let n = self.0[1].get_num_advances(n);
        let encode_third_rotor = self.0[2].encode_at(encode_second_rotor, n);
        encode_third_rotor
    }

    pub fn decode_at(&self, c: Character, n: usize) -> Character {
        let r1_advances = n;
        let r2_advances = self.0[0].get_num_advances(n);
        let r3_advances = self.0[1].get_num_advances(r2_advances);

        let decode_third_rotor = self.0[2].decode_at(c, r3_advances);
        let decode_second_rotor = self.0[1].decode_at(decode_third_rotor, r2_advances);
        let decode_first_rotor = self.0[0].decode_at(decode_second_rotor, r1_advances);
        decode_first_rotor
    }
}

impl TryFrom<[(Rotors, char); 3]> for RotorConfig {
    type Error = Bruh;

    fn try_from(value: [(Rotors, char); 3]) -> Result<Self, Self::Error> {
        let iter = value.iter();
        let rawcount = iter.clone().count();
        let unique = iter.clone().map(|(r, _)| r).unique().count();

        if rawcount == 3 && unique == 3 {
            let v: Vec<Rotor> = iter
                .clone()
                .map(|(r, c)| Rotor::try_from((*r, *c)))
                .try_collect()?;

            return Ok(RotorConfig(v));
        };
        Err(anyhow!("Invalid rotor configuration"))
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

    fn encode_at(&self, c: Character, n: usize) -> Character {
        //println!("{c} {n} {:?}", self.position);
        let offset: Position = self.position + n;
        self.cipher.encode(c + offset)
    }

    fn decode_at(&self, c: Character, n: usize) -> Character {
        let offset: Position = self.position + n;
        let dec = self.cipher.decode(c);

        //println!("{dec} {offset:?}");
        dec - offset
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

        let mut result = (r * self.notches.0.len()) + notches_left;

        if r > 0 {
            result = result + notches_past
        }

        result
    }
}

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
    use super::{Rotor, RotorConfig, Rotors};
    use crate::common::Character;

    #[test]
    fn rotorconfig_codec() {
        let _r = || Rotor::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ", &['A'], 'A').unwrap();

        let r1 = Rotor::try_from((Rotors::I, 'B')).unwrap();
        let r2 = Rotor::try_from((Rotors::IV, 'N')).unwrap();
        let r3 = Rotor::try_from((Rotors::III, 'X')).unwrap();

        let rc = RotorConfig(vec![r1, r2, r3]);

        let a = Character::try_from('A').unwrap();

        let t = |n: usize| {
            let ct = rc.encode_at(a, n);
            let pt = rc.decode_at(ct, n);
            assert_eq!(a, pt)
        };

        t(1);
        t(26);
        t(53);
        t(1_000_000);
    }

    #[test]
    fn num_advances() {
        let r = Rotor::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ", &['A'], 'A').unwrap();

        {
            let advances_lt_1_rot = r.get_num_advances(1);
            assert_eq!(1, advances_lt_1_rot);
        }

        {
            let advances_eq_1_rot = r.get_num_advances(26);
            assert_eq!(advances_eq_1_rot, 2)
        }

        {
            let advances_gt_1_rot = r.get_num_advances(53);
            assert_eq!(advances_gt_1_rot, 4)
        }
    }

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
        let _r = Rotor::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ", &['A'], 'B').unwrap();

        //EKMFLGDQVZNTOWYHXUSPAIBRCJ
        let r = Rotor::try_from((Rotors::I, 'B')).unwrap();
        (0..=1000).into_iter().for_each(|n| {
            ('A'..='Z')
                .into_iter()
                .map(|c| Character::try_from(c).unwrap())
                .for_each(|plaintext| {
                    let ciphertext = r.encode_at(plaintext, n.into());
                    let res = r.decode_at(ciphertext, n);

                    println!("{n}: {:?} {plaintext}-{ciphertext}-{res}", r.position);
                    // fix subtraction code

                    assert_eq!(plaintext, res);
                });
        })
    }
}
