#![allow(dead_code)]

pub struct Reflector {
    cipher: &'static str,
}

/// Struct used in the implementation of the rotor mechanism
impl Reflector {

    /// Generates a new reflector given a cipher
    /// The cipher must be reflective so that each substitution also works backwards
    fn new(cipher: &'static str) -> Reflector {
        Reflector { cipher }
    }

    /// Returns a pre-generated reflector given a member of the `ReflectorList` enum
    pub fn from(reflector: ReflectorList) -> Reflector{
        match reflector{
            ReflectorList::A => Reflector::new("EJMZALYXVBWFCRQUONTSPIKHGD"),
            ReflectorList::B => Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT"),
            ReflectorList::C => Reflector::new("FVPJIAOYEDRZXWGCTKUQSBNMHL"),
            ReflectorList::DEBUG => Reflector::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        }
    }

    /// Encodes a given char through the reflector.
    /// Given the properties of the reflector, if the output of this function was fed through this function,
    /// you would get back the original char
    pub fn encode(&self, c: char) -> char {
        const OFFSET: u8 = b'A';
        let index = c as u8 - OFFSET;
        self.cipher.chars().nth(index as usize).unwrap()
    }
}

/// This enum represents each available reflector on the enigma machine plus an extra reflector for debug purposes
/// Each reflector is a substitution cipher where the substitutions are reflective. For example, if a reflector
/// substitutes `A` with `Z` it also substitutes `Z` with `A`.  
/// The reflector does not move
pub enum ReflectorList{
    A,
    B,
    C,
    DEBUG
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reflector() {
        let current_reflector = Reflector::from(ReflectorList::A);
        ('A'..='Z').into_iter().for_each(|c| {
            let ciphertext = current_reflector.encode(c);
            let plaintext = current_reflector.encode(ciphertext);

            println!(
                "{}: Encodes to: {}, Decodes to: {}",
                c, ciphertext, plaintext
            );
            assert_eq!(c, plaintext);
        })
    }
}
