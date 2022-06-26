#![allow(dead_code)]

pub mod rotors {
    use crate::rotor::Rotor;
    pub const I: Rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", &['Q']);
    pub const II: Rotor = Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", &['E']);
    pub const III: Rotor = Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", &['V']);
    pub const IV: Rotor = Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", &['J']);
    pub const V: Rotor = Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", &['Z']);
    pub const VI: Rotor = Rotor::new("JPGVOUMFYQBENHZRDKASXLICTW", &['Z', 'M']);
    pub const VII: Rotor = Rotor::new("NZJHGRCXMYSWBOUFAIVLPEKQDT", &['Z', 'M']);
    pub const VIII: Rotor = Rotor::new("FKQHTLXOCBJSPDZRAMEWNIUYGV", &['Z', 'M']);
}

pub struct Rotor {
    cipher: &'static str,
    notch: &'static [char],
    position: char,
}

impl Rotor {
    const fn new(cipher: &'static str, notch: &'static [char]) -> Rotor {
        // TODO Ensure that cipher is only 26 chars long with valid A-Z values only
        Rotor {
            cipher,
            notch,
            position: 'A',
        }
    }

    fn set_position(&mut self, position: char) -> Result<(), &'static str> {
        match position {
            'A'..='Z' => {
                self.position = position;
                Ok(())
            }
            _ => Err("Position must be between A and Z"),
        }
    }

    pub fn rotate(&mut self) {
        match self.position {
            'A'..='Y' => self.position = (self.position as u8 + 1) as char,
            'Z' => self.position = 'A',
            _ => {
                panic!("By the time rotate is invoked, position should be a valid char between A-Z")
            }
        }
    }

    pub fn should_advance_next(&self)->bool{
        self.notch.iter().any(|notch|{
            let mut notch_position = *notch as u8 + 1;
            if notch_position >b'Z'{
                notch_position = b'A'
            }
            self.position == notch_position as char
        })
    }

    pub fn encode(&self, c: char) -> char {
        const OFFSET: u8 = b'A';
        let mut index = c as u8 - OFFSET + self.position as u8;

        if index >= b'Z' {
            index -= b'Z';
        }

        self.cipher.chars().nth(index as usize).unwrap()
    }

    pub fn decode(&self, c: char) -> char {
        const OFFSET: u8 = b'A';
        let position: u8 = self
            .cipher
            .chars()
            .position(|x| x == c)
            .unwrap()
            .try_into()
            .unwrap();
        let decoded: char = (position + OFFSET) as char;
        decoded
    }
}
