use crate::common::Character;
use anyhow::Context;
use bruh_moment::{bruh, Bruh};
use itertools::Itertools;
use nohash_hasher::BuildNoHashHasher;
use std::{collections::HashMap, str::FromStr};
use thiserror::Error;

pub trait Encode {
    fn encode(&self, c: Character) -> Character;
}

pub trait Decode {
    fn decode(&self, c: Character) -> Character;
}

pub struct Cipher(
    HashMap<Character, Character, BuildNoHashHasher<Character>>,
    HashMap<Character, Character, BuildNoHashHasher<Character>>,
);

impl Cipher {
    fn new() -> Cipher {
        let builder_l: BuildNoHashHasher<Character> = BuildNoHashHasher::default();
        let builder_r: BuildNoHashHasher<Character> = BuildNoHashHasher::default();
        let hash_l = HashMap::with_hasher(builder_l);
        let hash_r = HashMap::with_hasher(builder_r);
        Cipher(hash_l, hash_r)
    }
}

#[derive(Error, Debug)]
enum CipherError {
    #[error("Cipher does not contain enough unique characters(26) is a character duplicated in the cipher? Recieved")]
    Unique,
    #[error("Recieved {0}: Cipher may only contain 26 characters")]
    TooMany(usize),
    #[error("Recieved {0}: Cipher may only contain 26 characters")]
    TooFew(usize),
}

impl FromStr for Cipher {
    type Err = Bruh;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Vec<Character> = s
            .chars()
            .into_iter()
            .map(|c| Character::try_from(c))
            .try_collect()
            .with_context(|| format!("Tried to create a cipher from a string"))?;

        match s.len() {
            0..=25 => Err(bruh!(CipherError::TooFew(s.len()))),
            26 => Cipher::try_from(res),
            _ => Err(bruh!(CipherError::TooMany(s.len()))),
        }
    }
}

impl TryFrom<Vec<Character>> for Cipher {
    type Error = Bruh;

    fn try_from(value: Vec<Character>) -> Result<Self, Self::Error> {
        let res = value.iter().unique().count();
        match res {
            26 => ('A'..='Z')
                .into_iter()
                .map(|c| Character::try_from(c).unwrap())
                .enumerate()
                .fold(Ok(Cipher::new()), |acc, (i, next)| match acc {
                    Ok(mut cipher) => {
                        cipher.0.insert(next, value[i]);
                        cipher.1.insert(value[i], next);
                        Ok(cipher)
                    }
                    Err(_) => unreachable!(),
                }),
            _ => Err(bruh!(CipherError::Unique)),
        }
    }
}

impl Encode for Cipher {
    fn encode(&self, c: Character) -> Character {
        *self.0.get(&c).unwrap()
    }
}

impl Decode for Cipher {
    fn decode(&self, c: Character) -> Character {
        *self.1.get(&c).unwrap()
    }
}

#[cfg(test)]
mod tests_cipher {
    use std::str::FromStr;

    use crate::common::Character;

    use super::{Cipher, Decode, Encode};

    #[test]
    fn codec() {
        let cipher = Cipher::from_str("EKMFLGDQVZNTOWYHXUSPAIBRCJ").unwrap();
        ('A'..='Z')
            .into_iter()
            .map(|c| Character::try_from(c).unwrap())
            .for_each(|c| {
                let ciphertext = cipher.encode(c);
                let plaintext = cipher.decode(ciphertext);

                //println!("{}: {} {}", c, ciphertext, plaintext);
                assert_eq!(c, plaintext)
            })
    }

    /* #[test]
    fn encode() {
        let cipher = Cipher::from_str("ZYXWVUTSRQPONMLKJIHGFEDCBA").unwrap();
        ('A'..='Z')
            .into_iter()
            .map(|c| Character::try_from(c).unwrap())
            .for_each(|c| {
                let r = cipher.encode(c);

                let index: usize = (25 - r.get_offset()).into();
                let cmp = cipher.0.iter().nth(index).unwrap();

                println!("Input {}", c);
                println!("Output {}", r);
                println!("Index {}", index);
                println!("Cmp {}", *cmp);

                assert!(r == *cmp)
            })
    }

    #[test]
    fn decode() {
        let cipher = Cipher::from_str("ZYXWVUTSRQPONMLKJIHGFEDCBA").unwrap();
        ('A'..='Z')
            .into_iter()
            .map(|c| Character::try_from(c).unwrap())
            .for_each(|c| {
                let r = cipher.decode(c);

                let index: usize = (25 - r.get_offset()).into();
                let cmp = cipher.0.iter().nth(index).unwrap();

                println!("Input {}", c);
                println!("Output {}", r);
                println!("Index {}", index);
                println!("Cmp {}", *cmp);

                assert!(*cmp == r)
            })
    }*/

    #[test]
    fn length_too_small() {
        match Cipher::from_str("AZ") {
            Ok(_) => {
                panic!("Should not be able to construct cipher less than 26 chacaters in length")
            }
            Err(_) => (),
        }
    }

    #[test]
    fn length_too_big() {
        match Cipher::from_str("ZYXWVUTSRQPONMLKJIHGFEDCBAA") {
            Ok(_) => {
                panic!("Should not be able to construct cipher greater than 26 chacaters in length")
            }
            Err(_) => (),
        }
    }

    #[test]
    fn no_duplicates() {
        match Cipher::from_str("AAAAAAAAAAAAAAAAAAAAAAAAAA") {
            Ok(_) => panic!("Cipher should contain unique characters"),
            Err(_) => (),
        }
    }

    #[test]
    fn only_alphabetics() {
        match Cipher::from_str("1234567890*+-;'!@#$%^&*()_") {
            Ok(_) => {
                panic!("Should not be able to construct cipher with non-alphabetic characters")
            }
            Err(_) => (),
        }
    }
}
