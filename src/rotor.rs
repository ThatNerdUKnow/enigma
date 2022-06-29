#![allow(dead_code)]
use std::fmt::{self};
use itertools::Itertools;
use prae::Wrapper;

/// This enum represents each available rotor on the enigma machine plus an extra rotor used for debug purposes.
/// Each rotor is a simple substition cipher plus one or two notches which would allow the next rotor in the sequence to rotate
pub enum RotorList {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
    DEBUG,
}

/// Struct used in the implementation of the rotor mechanism
pub struct Rotor {
    cipher: RotorCipher,
    notch: &'static [char],
    position: char,
}

prae::define! {
    #[derive(Debug)]
    pub RotorCipher: &'static str;
    validate(CipherError) |cipher|{

        

        match cipher.len(){
            26 => (),
            _ => return Err(CipherError::Length)
        };

        match cipher.chars().unique().count() {
            26 => (),
            _=> return Err(CipherError::Unique)
        };
        
        let mapping = cipher.
        chars()
        .map(|c|{
            const OFFSET: u8 = b'A';
            let c = c as u8 - OFFSET;
            cipher
            .chars()
            .nth(c as usize)
            .unwrap()
        })
        .unique();

        match mapping.count(){
            26 => (),
            _ => return Err(CipherError::Mapping)
        };

        let charset = cipher.chars().all(|c| c >= 'A' && c <= 'Z');

        match charset{
            true => (),
            false => return Err(CipherError::Charset)
        }

        Ok(())
    };
}

impl Rotor {
    /// Generates a new Rotor.
    /// The cipher is a 26 character long string representing the substitution cipher for the rotor
    /// The cipher must only contain values `A-Z` inclusive and characters may not be repeated  
    /// The `position` param must only be a char between `A-Z`
    fn new(cipher: &'static str, notch: &'static [char], position: char) -> Rotor {
        // TODO Ensure that cipher is only 26 chars long with valid A-Z values only
        
        Rotor {
            cipher: RotorCipher::new(cipher).unwrap(),
            notch,
            position,
        }
    }

    /// Returns a pre-generated rotor given a member of the `RotorList` enum and a position between `A-Z`
    pub fn from(rotor_type: RotorList, position: char) -> Rotor {
        match rotor_type {
            RotorList::I => Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", &['Q'], position),
            RotorList::II => Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", &['E'], position),
            RotorList::III => Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", &['V'], position),
            RotorList::IV => Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", &['J'], position),
            RotorList::V => Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", &['Z'], position),
            RotorList::VI => Rotor::new("JPGVOUMFYQBENHZRDKASXLICTW", &['Z', 'M'], position),
            RotorList::VII => Rotor::new("NZJHGRCXMYSWBOUFAIVLPEKQDT", &['Z', 'M'], position),
            RotorList::VIII => Rotor::new("FKQHTLXOCBJSPDZRAMEWNIUYGV", &['Z', 'M'], position),
            RotorList::DEBUG => Rotor::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ", &[], position),
        }
    }

    /// Used to advance the position of the rotor, returns a char representing the rotor's final position
    pub fn rotate(&mut self) -> char {
        match self.position {
            'A'..='Y' => self.position = (self.position as u8 + 1) as char,
            'Z' => self.position = 'A',
            _ => {
                panic!("By the time rotate is invoked, position should be a valid char between A-Z")
            }
        }

        self.position
    }

    /// Used to determine if the next rotor in the sequence should rotate
    pub fn should_advance_next(&self) -> bool {
        self.notch.iter().any(|notch| {
            let mut notch_position = *notch as u8 + 1;
            if notch_position > b'Z' {
                notch_position = b'A'
            }
            self.position == notch_position as char
        })
    }

    /// Encodes the given char
    pub fn encode(&self, c: char) -> char {
        const OFFSET: u8 = b'A';
        let plaintext = c as u8 - OFFSET;
        let position = self.position as u8 - OFFSET;
        let mut index: u8 = plaintext + position;

        if index > b'Z' - OFFSET {
            index -= b'Z' - OFFSET + 1;
        }
        self.cipher.get().chars().nth(index as usize).unwrap()
    }

    /// Decodes the given char
    pub fn decode(&self, c: char) -> char {
        const OFFSET: u8 = b'A';
        let ciphertext: u8 = self // Index of first occurance of the given char in the cipher string
            .cipher
            .get()
            .chars()
            .position(|x| x == c)
            .unwrap()
            .try_into()
            .unwrap();

        let position_offset = self.position as u8 - OFFSET; // rotation index

        let mut decoded: u8 = (ciphertext + OFFSET) - position_offset;

        if ciphertext < position_offset {
            decoded = b'Z' - position_offset + ciphertext + 1
        }

        decoded as char
    }
}

pub enum CipherError{
    Length,
    Unique,
    Mapping,
    Charset
}

impl fmt::Debug for CipherError{
    fn fmt(&self, f:&mut fmt::Formatter)-> fmt::Result {
        match self{
            CipherError::Length => write!(f,"The given cipher must be exactly 26 characters").unwrap(),
            CipherError::Unique => write!(f,"Each character in the cipher may only appear once").unwrap(),
            CipherError::Mapping => write!(f,"Cipher encodes must be reversible. If A maps to Z, Z must also map to A").unwrap(),
            CipherError::Charset => write!(f,"Cipher may only include characters A-Z inclusive").unwrap()
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codec() {
        let current_rotor = Rotor::from(RotorList::DEBUG, 'C');
        ('A'..='Z').into_iter().for_each(|c| {
            let ciphertext = current_rotor.encode(c);
            let plaintext = current_rotor.decode(ciphertext);

            println!(
                "{}: Encodes to: {}, Decodes to: {}",
                c, ciphertext, plaintext
            );
            assert_eq!(c, plaintext);
        })
    }

    #[test]
    fn rotation() {
        let mut current_rotor = Rotor::from(RotorList::I, 'A');
        (0..=200).into_iter().for_each(|_| {
            let inputchar = current_rotor.position;

            let new_position = current_rotor.rotate();
            println!("{}: {}", inputchar, new_position);
            match inputchar {
                'A'..='Y' => assert_eq!(new_position, (inputchar as u8 + 1) as char),
                'Z' => assert_eq!(new_position, 'A'),
                _ => panic!("Position should only be between A-Z after rotation"),
            }
        })
    }

    #[test]
    fn should_advance_next() {
        let mut current_rotor = Rotor::from(RotorList::I, 'B');
        (0..26).into_iter().for_each(|x| {
            const OFFSET: u8 = b'A';
            let inputchar = (x + OFFSET) as char;
            println!("{}", inputchar);

            current_rotor.rotate();

            let does_advance_next = current_rotor.should_advance_next();
            current_rotor.notch.iter().for_each(|notch| {
                if current_rotor.position as u8 == *notch as u8 + 1 {
                    assert_eq!(true, does_advance_next)
                } else {
                    assert_eq!(false, does_advance_next)
                }
            })
        })
    }

    #[test]
    fn construct_i() {
        Rotor::from(RotorList::I, 'A');
    }

    #[test]
    fn construct_ii() {
        Rotor::from(RotorList::II, 'A');
    }

    #[test]
    fn construct_iii() {
        Rotor::from(RotorList::III, 'A');
    }

    #[test]
    fn construct_iv() {
        Rotor::from(RotorList::IV, 'A');
    }

    #[test]
    fn construct_v() {
        Rotor::from(RotorList::V, 'A');
    }

    #[test]
    fn construct_vi() {
        Rotor::from(RotorList::VI, 'A');
    }

    #[test]
    fn construct_vii() {
        Rotor::from(RotorList::VII, 'A');
    }

    #[test]
    fn construct_viii() {
        Rotor::from(RotorList::VIII, 'A');
    }

    #[test]
    fn construct_debug() {
        Rotor::from(RotorList::DEBUG, 'A');
    }
}
