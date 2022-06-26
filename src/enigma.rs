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
        self.advance_rotors();

        let rotor_pass = self.rotor_pass(c);

        let reflect_pass = self.reflect_pass(rotor_pass);

        let reverse_rotor_pass = self.reverse_rotor_pass(reflect_pass);
        reverse_rotor_pass
    }

    fn rotor_pass(&mut self, c: char) -> char {
        self.rotors
            .iter()
            .fold(c, |acc, current_rotor| current_rotor.encode(acc))
    }

    fn reflect_pass(&mut self, c: char) -> char {
        self.reflector.encode(c)
    }

    fn reverse_rotor_pass(&mut self, c: char) -> char {
        self.rotors
            .iter()
            .rev()
            .fold(c, |acc, current_rotor| current_rotor.decode(acc))
    }

    fn advance_rotors(&mut self) {
        self.rotors[0].rotate();

        let mut iterhandle = self.rotors.iter_mut().peekable();

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reflector::reflectors;
    use crate::rotor::RotorList;

    #[test]
    fn rotor_identity() {
        let rotor_config = vec![
            Rotor::from(RotorList::I, 'X'),
            Rotor::from(RotorList::V,'N'),
            Rotor::from(RotorList::VIII,'J')
        ];
        let reflector = reflectors::A;

        let mut enigma = Enigma::new(rotor_config, reflector);

        (b'A'..b'Z').into_iter().for_each(|x| {
            let rotor_pass = enigma.rotor_pass(x as char);
            let reflector_pass = enigma.reflect_pass(rotor_pass);
            let reverse_rotor_pass = enigma.reverse_rotor_pass(reflector_pass);

            println!(
                "Rotors: {}, Reflector: {}, Rev_Rotors: {}",
                rotor_pass, reflector_pass, reverse_rotor_pass
            );

            let rotor_pass_2 = enigma.rotor_pass(reverse_rotor_pass);
            let reflector_pass_2 = enigma.reflect_pass(rotor_pass_2);
            let rev2 = enigma.reverse_rotor_pass(reflector_pass_2);

            println!(
                "Rotors: {}, Reflector: {}, Rev_Rotors: {}",
                rotor_pass_2, reflector_pass_2, rev2
            );

            assert_eq!(x as char,rev2);
        })
    }
}
