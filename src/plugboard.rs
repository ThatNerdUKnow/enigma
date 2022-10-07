use crate::{
    cipher::{Cipher, Decode, Encode},
    common::Character,
};
use anyhow::{anyhow, Error, Ok};
use std::collections::{HashMap, HashSet};

pub struct Plugboard {
    cipher: Cipher,
}

pub struct Plugs(Vec<Plug>);

#[derive(Debug)]
pub struct Plug(Character, Character);

impl TryFrom<Plugs> for Plugboard {
    fn try_from(value: Plugs) -> Result<Plugboard, self::Error> {
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

    type Error = Error;
}

impl TryFrom<(Character, Character)> for Plug {
    type Error = Error;

    fn try_from(value: (Character, Character)) -> Result<Self, Self::Error> {
        match value.0 == value.1 {
            true => Err(anyhow!("Plug can not contain the same character")),
            false => Ok(Plug(value.0, value.1)),
        }
    }
}

impl TryFrom<Vec<Plug>> for Plugs {
    type Error = Error;

    fn try_from(value: Vec<Plug>) -> Result<Self, Self::Error> {
        match value.len() {
            0..=10 => (),
            _ => {
                return Err(anyhow!(
                    "No more than 10 plugs may be used in the plugboard"
                ))
            }
        }
        let set: HashSet<Character> = HashSet::new();
        let _plugs = value.iter().try_fold(&value, |acc, p| {
            match (set.contains(&p.0), set.contains(&p.1)) {
                (false, false) => Ok(acc),
                _ => Err(anyhow!(
                    "The same character can not be used for multiple plugs concurrently"
                )),
            }
        })?;

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
}
