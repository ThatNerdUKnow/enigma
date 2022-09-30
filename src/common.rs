use std::{ops::Add, str::FromStr, collections::HashSet};

#[derive(Debug,PartialEq, Eq,Hash,Clone, Copy)]
pub struct Character(char);
pub struct Position(u8);

impl TryFrom<char> for Character {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let value_uppercase = value.to_ascii_uppercase();
        match value_uppercase {
            'A'..='Z' => Ok(Character(value_uppercase)),
            _ => Err("Parsing error: only valid characters are A-Z"),
        }
    }
}

impl TryFrom<u8> for Position {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..=25 => Ok(Position(value)),
            _ => Err("Parsing error: only valid positions are 0..=25"),
        }
    }
}

impl Add<u8> for Position {
    type Output = Position;

    fn add(self, rhs: u8) -> Self::Output {
        let offset = (self.0 + rhs) % 26;
        Position(offset)
    }
}

impl Position{
    pub fn advance(mut self){
        self = self + 1_u8;
    }
}


pub trait Encode{
    fn encode(self,c:Character)->Character;
}

pub trait Decode{
    fn decode(self,c:Character)->Character;
}