use std::str::FromStr;

use crate::{
    cipher::{Cipher, Decode, Encode},
    common::Character,
};

struct Reflector {
    cipher: Cipher,
}

enum Reflectors {
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
    fn from(r: Reflectors) -> Self {
        match r {
            Reflectors::A => Reflector::new("EJMZALYXVBWFCRQUONTSPIKHGD"),
            Reflectors::B => Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT"),
            Reflectors::C => Reflector::new("FVPJIAOYEDRZXWGCTKUQSBNMHL"),
        }
    }
}

impl Encode for Reflector {
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
