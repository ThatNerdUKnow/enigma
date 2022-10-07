use crate::{
    cipher::{Cipher, Decode, Encode},
    common::Character,
};
use anyhow::Error;
use itertools::Itertools;
use std::collections::HashMap;
use thiserror::Error;

pub struct Plugboard {
    cipher: Cipher,
}

#[derive(Debug)]
pub struct Plugs(Vec<Plug>);

#[derive(Debug)]
pub struct Plug(Character, Character);

#[derive(Error, Debug)]
pub enum PlugboardError {
    #[error("Recieved {0} plugs, No more than 10 plugs may be used in the plugboard")]
    TooMany(usize),
    #[error("Can not map multiple plugs to the same character")]
    Mapping,
    #[error("Can not map a character to itself")]
    Duplicate,
}

impl TryFrom<Plugs> for Plugboard {
    type Error = Error;
    fn try_from(value: Plugs) -> Result<Plugboard, self::Error> {
        // not my best work
        let passthrough = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .map(|c| Character::try_from(c).unwrap());

        let alphabet: HashMap<Character, Character> = passthrough.clone().fold(
            HashMap::new(),
            |mut acc: HashMap<Character, Character>, next| {
                acc.insert(next, next);
                acc
            },
        ); // collect into a bihashmap

        let cipher = value.0.iter().fold(
            alphabet,
            |mut acc: HashMap<Character, Character>, next: &Plug| {
                acc.insert(next.0, next.1);
                acc.insert(next.1, next.0);
                acc
            },
        );

        let x: Vec<Character> = passthrough
            .fold(Vec::new(), |mut acc, next| {
                let substitution = cipher.get(&next).unwrap();
                acc.push(substitution);
                acc
            })
            .into_iter()
            .cloned()
            .collect();

        let cipher: Cipher = Cipher::try_from(x)?;
        Ok(Plugboard { cipher: cipher })
    }
}

impl TryFrom<(Character, Character)> for Plug {
    type Error = PlugboardError;

    fn try_from(value: (Character, Character)) -> Result<Self, Self::Error> {
        match value.0 == value.1 {
            true => Err(PlugboardError::Duplicate),
            false => Ok(Plug(value.0, value.1)),
        }
    }
}

impl TryFrom<Vec<Plug>> for Plugs {
    type Error = PlugboardError;

    fn try_from(value: Vec<Plug>) -> Result<Self, Self::Error> {
        let len = value.len();
        match len {
            0..=10 => (),
            _ => return Err(PlugboardError::TooMany(value.len())),
        }

        let uniquechars = value
            .iter()
            .fold(Vec::new(), |mut acc: Vec<Character>, next| {
                acc.push(next.0);
                acc.push(next.1);
                acc
            })
            .iter()
            .unique()
            .count();

        if uniquechars != 2 * len {
            return Err(PlugboardError::Mapping);
        }

        Ok(Plugs(value))
    }
}

impl Encode for Plugboard {
    fn encode(&self, c: Character) -> Character {
        self.cipher.encode(c)
    }
}

impl Decode for Plugboard {
    fn decode(&self, c: Character) -> Character {
        self.cipher.decode(c)
    }
}

#[cfg(test)]
mod tests {
    use crate::{cipher::Encode, common::Character};

    use super::{Plug, Plugboard, Plugs};

    #[test]
    fn empty_vec_cons() {
        let _ =
            Plugs::try_from(Vec::new()).expect("Should be able to construct an empty plugboard");
    }

    #[test]
    fn encode() {
        let pb = Plugboard::try_from(Plugs::try_from(Vec::new()).unwrap())
            .expect("Should be able to construct an empty plugboard");

        ('A'..='Z')
            .into_iter()
            .map(|c| Character::try_from(c).unwrap())
            .for_each(|c| {
                let ct = pb.encode(c);
                assert_eq!(ct, c)
            })
    }

    #[test]
    fn reflect() {
        let v: Vec<Plug> = [
            ('A', 'B'),
            ('C', 'Z'),
            ('X', 'L'),
            ('N', 'P'),
            ('Q', 'U'),
            ('M', 'O'),
            ('I', 'J'),
            ('S', 'V'),
            ('H', 'G'),
            ('D', 'R'),
        ]
        .iter()
        .map(|x| {
            (
                Character::try_from(x.0).unwrap(),
                Character::try_from(x.1).unwrap(),
            )
        })
        .map(|p| Plug::try_from(p).unwrap())
        .collect();

        let pb = Plugboard::try_from(Plugs::try_from(v).unwrap())
            .expect("Should be able to construct an empty plugboard");

        ('A'..='Z')
            .into_iter()
            .map(|c| Character::try_from(c).unwrap())
            .for_each(|c| {
                let ciphertext = pb.encode(c);
                let plaintext = pb.encode(ciphertext);
                println!("{}: {},{}", c, ciphertext, plaintext);
                assert_eq!(plaintext, c)
            })
    }

    #[test]
    fn too_many() {
        let plugs: Vec<Plug> = [
            ('A', 'B'),
            ('C', 'Z'),
            ('X', 'L'),
            ('N', 'P'),
            ('Q', 'U'),
            ('M', 'O'),
            ('I', 'J'),
            ('S', 'V'),
            ('H', 'G'),
            ('D', 'R'),
            ('Y', 'F'),
        ]
        .iter()
        .map(|x| {
            (
                Character::try_from(x.0).unwrap(),
                Character::try_from(x.1).unwrap(),
            )
        })
        .map(|p| Plug::try_from(p).unwrap())
        .collect();

        match Plugs::try_from(plugs) {
            Ok(_) => panic!("Should not be able to construct with more than 10 plugs"),
            Err(_) => (),
        }
    }

    #[test]
    fn same_character() {
        let c = Character::try_from('A').unwrap();
        let x = Plug::try_from((c, c));

        match x {
            Ok(_) => {
                panic!("Should not be able to construct a plug containing duplicate characters")
            }
            Err(_) => (),
        }
    }

    #[test]
    fn duplicate_mapping() {
        let a = Character::try_from('A').unwrap();
        let b = Character::try_from('B').unwrap();
        let c = Character::try_from('C').unwrap();

        let p1 = Plug::try_from((a, b)).unwrap();
        let p2 = Plug::try_from((a, c)).unwrap();

        match Plugs::try_from(vec![p1, p2]) {
            Ok(plugs) => panic!("Recieved {plugs:?}: Should not be able to construct plugs that contain non-unique mappings"),
            Err(_) => (),
        }
    }
}
