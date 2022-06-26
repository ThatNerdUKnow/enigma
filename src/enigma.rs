use crate::reflector::Reflector;
use crate::rotor::Rotor;

pub(crate) struct Enigma {
    reflector: Reflector,
    rotors: Vec<Rotor>,
}

impl Enigma {
    pub fn new(rotors: Vec<Rotor>, reflector: Reflector) -> Enigma {
        Enigma { rotors, reflector }
    }

    pub fn encode(&mut self, message: String) -> String {
        let cipher_text = message.chars().map(|c| self.encode_char(c)).collect();
        cipher_text
    }

    fn encode_char(&mut self, c: char) -> char {
        self.rotors[0].rotate();

        let mut iterhandle = self.rotors.iter_mut().peekable();

        /*for el in iterhandle {
            match iterhandle.peek_mut() {
                Some(next_rotor) => match el.should_advance_next() {
                    true => next_rotor.rotate(),
                    false => (),
                },
                None => (),
            }
        }*/

        while let Some(el) = iterhandle.next(){
            match iterhandle.peek_mut(){
                Some(next_rotor)=> match el.should_advance_next(){
                    true => next_rotor.rotate(),
                    false => ()
                },
                None => ()
            }
        }



        let first_pass = self
            .rotors
            .iter()
            .fold(c, |acc, current_rotor| current_rotor.encode(acc));

        let reflect_pass = self.reflector.encode(first_pass);

        let last_rotor_pass = self
            .rotors
            .iter()
            .fold(reflect_pass, |acc, current_rotor| current_rotor.encode(acc));

        last_rotor_pass
    }
}
