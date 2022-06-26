#![allow(dead_code)]

pub struct Reflector {
    cipher: &'static str,
}

impl Reflector {
    const fn new(cipher: &'static str) -> Reflector {
        Reflector { cipher }
    }

    pub fn encode(&self, c: char) -> char {
        const OFFSET: u8 = b'A';
        let index = c as u8 - OFFSET;
        self.cipher.chars().nth(index as usize).unwrap()
    }
}

pub mod reflectors {
    use crate::reflector::Reflector;
    pub const A: Reflector = Reflector::new("EJMZALYXVBWFCRQUONTSPIKHGD");
    pub const B: Reflector = Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT");
    pub const C: Reflector = Reflector::new("FVPJIAOYEDRZXWGCTKUQSBNMHL");
}
