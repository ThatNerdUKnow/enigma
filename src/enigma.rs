use std::fmt;

use crate::reflector::Reflector;
use crate::rotor::Rotor;
use itertools::Itertools;
use prae::Wrapper;

pub struct Enigma {
    reflector: Reflector,
    rotors: RotorConfig,
}

prae::define! {
    #[derive(Debug)]
    RotorConfig: Vec<Rotor>;
    validate(RotorConfigError) |config|{

        println!("{}",config.iter().unique().count());

        match config.len(){
            3..=4 => (),
            _ => return Err(RotorConfigError::Size)
        }

        match config.iter().unique().count(){
            3..=4 =>(),
            _ => return Err(RotorConfigError::Duplicate)
        }

        Ok(())
    };
}

impl RotorConfig{
    fn advance_rotors(&mut self)
    {
        self.0[0].rotate();

        let mut iterhandle = self.0.iter_mut().peekable();

        while let Some(el) = iterhandle.next() {
            match iterhandle.peek_mut() {
                Some(next_rotor) => match el.should_advance_next() {
                    true => {
                        next_rotor.rotate();
                    }
                    false => (),
                },
                None => (),
            }
        }
    }

}

pub enum RotorConfigError{
    Duplicate,
    Size
}

impl fmt::Debug for RotorConfigError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Duplicate => write!(f, "You may not use duplicate rotors"),
            Self::Size => write!(f, "You may insert no more than 4 rotors and no less than 3 rotors"),
        }
    }
}

impl Enigma {

    /// Returns a new Enigma instance
    pub fn new(rotors: Vec<Rotor>, reflector: Reflector) -> Enigma {
        Enigma { rotors:RotorConfig::new(rotors).unwrap(), reflector }
    }

    /// Encodes an entire string. Note that the only allowed characters are `A-Z` inclusive
    pub fn encode(&mut self, message: String) -> String {
        let cipher_text = message.chars().map(|c| self.encode_char(c)).collect();
        cipher_text
    }

    /// Encodes a single char
    fn encode_char(&mut self, c: char) -> char {
        self.advance_rotors();

        let rotor_pass = self.rotor_encode(c);

        let reflect_pass = self.reflect(rotor_pass);

        let reverse_rotor_pass = self.rotor_decode(reflect_pass);
        reverse_rotor_pass
    }

    /// This is the first encoding pass of the rotor mechanism
    fn rotor_encode(&mut self, c: char) -> char {
        self.rotors
            .get()
            .iter()
            .fold(c, |acc, current_rotor| current_rotor.encode(acc))
    }

    /// The result of the first pass of the rotor mechanism is sent through the reflector.
    fn reflect(&mut self, c: char) -> char {
        self.reflector.encode(c)
    }

    /// The character comes out of the reflector and then backwards through the rotor mechanism
    /// This property allows the enigma machine to be used as both an encoder and decoder
    fn rotor_decode(&mut self, c: char) -> char {
        self.rotors
            .get()
            .iter()
            .rev()
            .fold(c, |acc, current_rotor| current_rotor.decode(acc))
    }

    /// Advances the first rotor and then checks if subsequent rotors would rotate along
    fn advance_rotors(&mut self) {
        self.rotors.advance_rotors()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reflector::ReflectorList;
    use crate::rotor::RotorList;

    #[test]
    fn rotor_identity() {
        let rotor_config = vec![
            Rotor::from(RotorList::IV, 'X'),
            Rotor::from(RotorList::V,'N'),
            Rotor::from(RotorList::VIII,'J')
        ];
        let reflector = Reflector::from(ReflectorList::A);

        let mut enigma = Enigma::new(rotor_config, reflector);

        (b'A'..b'Z').into_iter().for_each(|x| {
            let rotor_pass = enigma.rotor_encode(x as char);
            let reflector_pass = enigma.reflect(rotor_pass);
            let reverse_rotor_pass = enigma.rotor_decode(reflector_pass);

            println!(
                "Rotors: {}, Reflector: {}, Rev_Rotors: {}",
                rotor_pass, reflector_pass, reverse_rotor_pass
            );

            let rotor_pass_2 = enigma.rotor_encode(reverse_rotor_pass);
            let reflector_pass_2 = enigma.reflect(rotor_pass_2);
            let rev2 = enigma.rotor_decode(reflector_pass_2);

            println!(
                "Rotors: {}, Reflector: {}, Rev_Rotors: {}",
                rotor_pass_2, reflector_pass_2, rev2
            );

            assert_eq!(x as char,rev2);
        })
    }

    #[test]
    fn rotor_config_size_bound(){
        println!("Will pass if given a RotorConfig which is either too big or too small");
        RotorConfig::new(vec![]).unwrap_err();
    }

    #[test]
    fn rotor_config_duplicate_bound(){
        println!("Will pass if given a RotorConfig which contains non-unique rotors");
        let rotor_config = vec![
            Rotor::from(RotorList::V, 'X'),
            Rotor::from(RotorList::V,'N'),
            Rotor::from(RotorList::VIII,'J')
        ];
        RotorConfig::new(rotor_config).unwrap_err();
    }
}
