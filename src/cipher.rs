use anyhow::{anyhow, Error};
use itertools::Itertools;

use crate::common::Character;
use std::str::FromStr;

pub trait Encode {
    fn encode(&self, c: Character) -> Character;
}

pub trait Decode {
    fn decode(&self, c: Character) -> Character;
}

pub struct Cipher(Vec<Character>);

impl Cipher {
    fn new() -> Cipher {
        Cipher(Vec::new())
    }
}

impl FromStr for Cipher {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Vec<Character> = s
            .chars()
            .into_iter()
            .map(|c| Character::try_from(c))
            .try_collect()?;

        match s.len() {
            0..=25 => Err(anyhow!(
                "Too few characters were provided in the cipher string {}",
                s
            )),
            26 => Cipher::try_from(res),
            _ => Err(anyhow!(
                "Too many characters were provided in the cipher string {}",
                s
            )),
        }
    }
}

impl TryFrom<Vec<Character>> for Cipher {
    type Error = Error;

    fn try_from(value: Vec<Character>) -> Result<Self, Self::Error> {
        let res = value.iter().unique().count();
        match res {
            26 => Ok(Cipher(value)),
            _ => Err(anyhow!("Parsing error: Cipher does not contain enough unique characters(26) Is a character duplicated in the cipher?")),
        }
    }
}

impl Encode for Cipher {
    fn encode(&self, c: Character) -> Character {
        self.0[c.get_offset() as usize]
    }
}

impl Decode for Cipher {
    fn decode(&self, c: Character) -> Character {
        let index = self.0.iter().position(|&f| f == c).unwrap();
        Character::try_from((b'A' + index as u8) as char).unwrap()
    }
}

#[cfg(test)]
mod tests_cipher {
    use std::str::FromStr;

    use crate::common::Character;

    use super::{Cipher, Decode, Encode};

    #[test]
    fn codec() {
        let cipher = Cipher::from_str("ZYXWVUTSRQPONMLKJIHGFEDCBA").unwrap();
        ('A'..='Z')
            .into_iter()
            .map(|c| Character::try_from(c).unwrap())
            .for_each(|c| {
                let ciphertext = cipher.encode(c);
                let plaintext = cipher.decode(ciphertext);

                println!("{}: {} {}", c, ciphertext, plaintext);
                assert!(c == plaintext)
            })
    }

    #[test]
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
    }

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
