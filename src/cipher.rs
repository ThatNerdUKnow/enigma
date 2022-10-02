use crate::common::Character;
use std::{collections::HashSet, str::FromStr};

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
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cipher: Cipher = Cipher::new();
        let mut set: HashSet<Character> = HashSet::new();
        s.chars()
            .into_iter()
            .map(|c| Character::try_from(c))
            .for_each(|c| {
                match c {
                    Ok(value) => {
                        set.insert(value);
                        cipher.0.push(value)
                    }
                    Err(e) => panic!("{:?}", e),
                };
            });

        match set.len(){
                26 => Ok(cipher),
                _ => Err("Parsing error: cipher does not contain enough characters(26) Is a character duplicated in the cipher?")
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
}
