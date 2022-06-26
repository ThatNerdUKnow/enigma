#![allow(dead_code)]
/*pub mod rotors {
    use crate::rotor::Rotor;
    pub const I: Rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", &['Q']);
    pub const II: Rotor = Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", &['E']);
    pub const III: Rotor = Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", &['V']);
    pub const IV: Rotor = Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", &['J']);
    pub const V: Rotor = Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", &['Z']);
    pub const VI: Rotor = Rotor::new("JPGVOUMFYQBENHZRDKASXLICTW", &['Z', 'M']);
    pub const VII: Rotor = Rotor::new("NZJHGRCXMYSWBOUFAIVLPEKQDT", &['Z', 'M']);
    pub const VIII: Rotor = Rotor::new("FKQHTLXOCBJSPDZRAMEWNIUYGV", &['Z', 'M']);
    pub const DEBUG: Rotor = Rotor::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ", &[]);
}*/

pub enum rotors {
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

#[derive(Clone)]
pub struct Rotor {
    cipher: &'static str,
    notch: &'static [char],
    position: char,
}

impl Rotor {
    fn new(cipher: &'static str, notch: &'static [char], position: char) -> Rotor {
        // TODO Ensure that cipher is only 26 chars long with valid A-Z values only
        Rotor {
            cipher,
            notch,
            position,
        }
    }

    pub fn from(rotor_type: rotors, position: char) -> Rotor {
        match rotor_type {
            rotors::I => Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", &['Q'], position),
            rotors::II => Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", &['E'], position),
            rotors::III => Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", &['V'], position),
            rotors::IV => Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", &['J'], position),
            rotors::V => Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", &['Z'], position),
            rotors::VI => Rotor::new("JPGVOUMFYQBENHZRDKASXLICTW", &['Z', 'M'], position),
            rotors::VII => Rotor::new("NZJHGRCXMYSWBOUFAIVLPEKQDT", &['Z', 'M'], position),
            rotors::VIII => Rotor::new("FKQHTLXOCBJSPDZRAMEWNIUYGV", &['Z', 'M'], position),
            rotors::DEBUG => Rotor::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ", &[], position),
        }
    }

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

    pub fn should_advance_next(&self) -> bool {
        self.notch.iter().any(|notch| {
            let mut notch_position = *notch as u8 + 1;
            if notch_position > b'Z' {
                notch_position = b'A'
            }
            self.position == notch_position as char
        })
    }

    pub fn encode(&self, c: char) -> char {
        const OFFSET: u8 = b'A';
        let plaintext = c as u8 - OFFSET;
        let position = self.position as u8 - OFFSET;
        let mut index: u8 = plaintext + position;

        if index > b'Z' - OFFSET {
            index -= b'Z' - OFFSET + 1;
        }
        self.cipher.chars().nth(index as usize).unwrap()
    }

    pub fn decode(&self, c: char) -> char {
        const OFFSET: u8 = b'A';
        let ciphertext: u8 = self // Index of first occurance of the given char in the cipher string
            .cipher
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codec() {
        let current_rotor = Rotor::from(rotors::DEBUG,'C');
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
        let mut current_rotor = Rotor::from(rotors::I,'X');
        (0..26).into_iter().for_each(|x| {
            const OFFSET: u8 = b'A';
            let inputchar = (x + OFFSET) as char;
            println!("{}", inputchar);

            let new_position = current_rotor.rotate();

            match inputchar {
                'A'..='Y' => assert_eq!(new_position, (inputchar as u8 + 1) as char),
                'Z' => assert_eq!(new_position, 'A'),
                _ => panic!("Position should only be between A-Z after rotation"),
            }
        })
    }

    #[test]
    fn should_advance_next() {
        let mut current_rotor = Rotor::from(rotors::I,'B');
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
}
