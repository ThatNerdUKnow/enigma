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

        self.advance_rotors();

        let first_pass = self
            .rotors
            .iter()
            .fold(c, |acc, current_rotor| current_rotor.encode(acc));

        let reflect_pass = self.reflector.encode(first_pass);

        let reverse_rotor_pass = self
            .rotors
            .iter()
            .fold(reflect_pass, |acc, current_rotor| current_rotor.decode(acc));
        reverse_rotor_pass
    }

    fn advance_rotors(&mut self){
        let mut iterhandle = self.rotors.iter_mut().peekable();

        while let Some(el) = iterhandle.next(){
            match iterhandle.peek_mut(){
                Some(next_rotor)=> match el.should_advance_next(){
                    true => {next_rotor.rotate();},
                    false => ()
                },
                None => ()
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::rotor::rotors;
    use crate::reflector::reflectors;
    use super::*;

    #[test]
    #[ignore]
    fn codec_e2e(){
        let rotor_config = vec![Rotor::from(rotors::DEBUG,'A')];
        let rotor_config_2 = rotor_config.clone();
        let message = "TESTINGTESTINGONETWOTHREE";
        let mut enigma_sender = Enigma::new(rotor_config,reflectors::DEBUG);
        let mut enigma_reciever = Enigma::new(rotor_config_2,reflectors::DEBUG);

        let ciphertext = enigma_sender.encode(message.to_string());
        let plaintext = enigma_reciever.encode(ciphertext);
        assert_eq!(plaintext,message)
    }
}
