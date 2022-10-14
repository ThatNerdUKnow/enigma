use std::str::FromStr;

use crate::{
    cipher::{Cipher, Encode},
    common::Character,
};

pub struct Reflector {
    cipher: Cipher,
}

/// This enum represents each available reflector on the enigma machine
/// Each reflector is a substitution cipher where the substitutions are reflective. For example, if a reflector
/// substitutes `A` with `Z` it also substitutes `Z` with `A`.  
/// The reflector does not move
pub enum Reflectors {
    A,
    B,
    C,
}

impl Reflector {
    fn new(s: &str) -> Reflector {
        let cipher = Cipher::from_str(s).unwrap();
        Reflector { cipher: cipher }
    }
}

impl From<Reflectors> for Reflector {
    /// Generates a new reflector given a cipher
    /// The cipher must be reflective so that each substitution also works backwards
    /// Returns a pre-generated reflector given a member of the `Reflectors` enum
    fn from(r: Reflectors) -> Self {
        match r {
            Reflectors::A => Reflector::new("EJMZALYXVBWFCRQUONTSPIKHGD"),
            Reflectors::B => Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT"),
            Reflectors::C => Reflector::new("FVPJIAOYEDRZXWGCTKUQSBNMHL"),
        }
    }
}

impl Encode for Reflector {
    /// Encodes a given char through the reflector.
    /// Given the properties of the reflector, if the output of this function was fed through this function,
    /// you would get back the original char
    fn encode(&self, c: Character) -> Character {
        self.cipher.encode(c)
    }
}

#[cfg(test)]
mod tests {
    use crate::{cipher::Encode, common::Character};

    use super::{Reflector, Reflectors};

    #[test]
    fn construct_a() {
        let _ = Reflector::from(Reflectors::A);
    }

    #[test]
    fn construct_b() {
        let _ = Reflector::from(Reflectors::B);
    }

    #[test]
    fn construct_c() {
        let _ = Reflector::from(Reflectors::C);
    }

    #[test]
    fn codec() {
        let reflector = Reflector::from(Reflectors::A);

        ('A'..='Z')
            .into_iter()
            .map(|c| Character::try_from(c).unwrap())
            .for_each(|c| {
                let ciphertext = reflector.encode(c);
                let plaintext = reflector.encode(ciphertext);

                assert_eq!(c, plaintext)
            })
    }
}
