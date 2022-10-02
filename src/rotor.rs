use crate::{
    cipher::{Cipher, Encode, Decode},
    common::{Position},
};
use strum_macros::EnumString;

#[derive(EnumString)]
enum Rotors {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
}
struct Rotor {
    position: Position,
    cipher: Cipher,
    turnover_positions: Vec<Position>,
}

impl TryFrom<Rotors> for Rotor {
    type Error = &'static str;

    fn try_from(value: Rotors) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl Encode for Rotor {
    fn encode(&self, c: crate::common::Character) -> crate::common::Character {
        let p = self.position;
        self.cipher.encode(c + p)
    }
}

impl Decode for Rotor {
    fn decode(&self, c: crate::common::Character) -> crate::common::Character {
        let p = self.position;
        self.cipher.decode(c + p)
    }
}
