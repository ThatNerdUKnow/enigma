use std::{collections::HashSet, str::FromStr, hash::Hash};
use crate::common::Character;

pub struct Cipher(Vec<Character>);

impl Cipher {
    fn new() ->Cipher {
        Cipher(Vec::new())
    }
}

impl FromStr for Cipher {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cipher:Cipher= Cipher::new();
        let mut set: HashSet<Character> = HashSet::new();
        s.chars()
            .into_iter()
            .map(|c| Character::try_from(c))
            .for_each(|c| { match c {
                Ok(value) => {
                    set.insert(value);
                    cipher.0.push(value)
                },
                Err(e) => panic!("{:?}",e),
            }; });

            match set.len(){
                26 => Ok(cipher),
                _ => Err("Parsing error: cipher does not contain enough characters(26) Is a character duplicated in the cipher?")
            }
    }
}