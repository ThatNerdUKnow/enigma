use bimap::BiMap;

use crate::{
    cipher::{Cipher, Decode, Encode},
    common::Character,
};
use anyhow::{anyhow, Error};
use std::collections::HashSet;

pub struct Plugboard {
    cipher: Cipher,
}

pub struct Plugs(Vec<Plug>);

pub struct Plug(Character, Character);

impl TryFrom<Plugs> for Plugboard {
    fn try_from(value: Plugs) -> Result<Plugboard, self::Error> {
        let passthrough = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .map(|c| Character::try_from(c).unwrap());

        let alphabet: BiMap<Character, Character> = passthrough.clone().fold(
            BiMap::new(),
            |mut acc: BiMap<Character, Character>, next| {
                acc.insert(next, next);
                acc
            },
        ); // collect into a bihashmap

        let cipher = value.0.iter().fold(
            alphabet,
            |mut acc: BiMap<Character, Character>, next: &Plug| {
                acc.insert(next.0, next.1);
                acc
            },
        );

        let x: Vec<Character> = passthrough
            .fold(Vec::new(), |mut acc, next| {
                let substitution = cipher.get_by_left(&next).unwrap();
                acc.push(substitution);
                acc
            })
            .into_iter()
            .cloned()
            .collect();

        let cipher: Cipher = Cipher::try_from(x)?;
        Ok(Plugboard { cipher: cipher })
    }

    type Error = Error;
}

impl<'a> From<(Character, Character)> for Plug {
    fn from(x: (Character, Character)) -> Self {
        Plug(x.0, x.1)
    }
}

impl TryFrom<Vec<Plug>> for Plugs {
    type Error = Error;

    fn try_from(value: Vec<Plug>) -> Result<Self, Self::Error> {
        let set: HashSet<Character> = HashSet::new();
        let plugs = value.iter().try_fold(&value, |acc, p| {
            match (set.contains(&p.0), set.contains(&p.1)) {
                (false, false) => Ok(acc),
                _ => Err(anyhow!(
                    "The same character can not be used for multiple plugs concurrently"
                )),
            }
        });

        match plugs {
            Ok(_) => Ok(Plugs(value)),
            Err(e) => Err(e),
        }
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
