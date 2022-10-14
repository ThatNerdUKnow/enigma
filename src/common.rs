use std::{
    fmt::{Display, Formatter},
    ops::{Add, Sub},
};

use nohash_hasher::IsEnabled;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Character(char);

impl IsEnabled for Character {}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, PartialOrd)]
pub struct Position(u8);

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("Recieved {0}: Only valid chars are A-Z")]
    Charset(char),
    #[error("Recieved {0}: Only valid positions are 0-25")]
    InvalidPosition(usize),
}

impl TryFrom<char> for Character {
    type Error = ParsingError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let value_uppercase = value.to_ascii_uppercase();
        match value_uppercase {
            'A'..='Z' => Ok(Character(value_uppercase)),
            _ => Err(ParsingError::Charset(value)),
        }
    }
}

impl TryFrom<u8> for Position {
    type Error = ParsingError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..=25 => Ok(Position(value)),
            _ => Err(ParsingError::InvalidPosition(value as usize)),
        }
    }
}

impl TryFrom<char> for Position {
    type Error = ParsingError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let v = value.to_ascii_uppercase();
        let offset = v as u8 - b'A';
        match Position::try_from(offset) {
            Ok(p) => Ok(p),
            Err(_) => Err(ParsingError::Charset(value)),
        }
    }
}

impl Add<usize> for Position {
    type Output = Position;

    fn add(self, rhs: usize) -> Self::Output {
        let offset = (self.0 as usize + rhs) % 26;
        Position((offset as usize).try_into().unwrap())
    }
}

impl Sub<Position> for Character {
    type Output = Self;

    fn sub(self, rhs: Position) -> Self::Output {
        // -25..=-1 if rhs is bigger, otherwise 0..=25
        let offset: isize = (self.get_offset() as isize - rhs.0 as isize) % 26;

        let result = match offset {
            -25..=-1 => (26 + offset) as u8 + b'A',
            0..=25 => offset as u8 + b'A',
            _ => unreachable!(),
        };

        /*#[cfg(test)]
        println!(
            "SUB<Position> for Character: self:{self} rhs:{rhs:?} offset:{offset} result:{result}"
        );*/
        Character::try_from(result as char).unwrap()
    }
}

impl Add<Position> for Character {
    type Output = Character;

    fn add(self, rhs: Position) -> Self::Output {
        let offset: u8 = (rhs.0 + self.get_offset()) % 26 + b'A';
        Character::try_from(offset as char).unwrap()
    }
}

impl Position {
    pub fn advance(&mut self) {
        self.0 = self.0 + 1_u8
    }
}

impl Character {
    pub fn get_offset(&self) -> u8 {
        self.0 as u8 - b'A'
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl Into<char> for Character {
    fn into(self) -> char {
        self.0
    }
}

#[cfg(test)]
mod tests_character {

    use super::Character;
    use super::Position;

    #[test]
    fn construct_uppercase() {
        ('A'..='Z').into_iter().for_each(|c| {
            Character::try_from(c).unwrap();
        })
    }

    #[test]
    fn construct_lowercase() {
        ('a'..='z').into_iter().for_each(|c| {
            Character::try_from(c).unwrap();
        })
    }

    #[test]
    fn add_position() {
        ('A'..='Z')
            .into_iter()
            .map(|c| Character::try_from(c).unwrap())
            .for_each(|c| {
                (0..25)
                    .into_iter()
                    .map(|n| Position::try_from(n).unwrap())
                    .for_each(|n| {
                        // Add<Position> for Character calls .unwrap() on the returned type
                        // Meaning that any invalid values should be caught
                        let _ = c + n;
                    })
            })
    }

    #[test]
    fn sub_position() {
        let chars: Vec<Character> = ('A'..='Z')
            .into_iter()
            .map(|c| Character::try_from(c).unwrap())
            .collect();

        let positions: Vec<Position> = (0..=25)
            .into_iter()
            .map(|n| Position::try_from(n).unwrap())
            .collect();

        let expected_val = |c: isize, n: isize| -> usize {
            if c >= n {
                (c - n).try_into().unwrap()
            } else {
                (26 + c - n).try_into().unwrap()
            }
        };

        let t = |c: char, n: isize| {
            let c: isize = (c as u8 - b'A').into();
            let r: Character = chars[c as usize] - positions[n as usize];
            let i = expected_val(c, n);
            println!("{c} {r} {i}");
            assert_eq!(r, chars[i])
        };

        (0..=25)
            .into_iter()
            .for_each(|n| ('A'..='Z').into_iter().for_each(|c| t(c, n)))
    }

    #[test]
    fn ca_pz_spot_test() {
        let p = Position(25);
        let c = Character('A');
        assert!(c + p == Character('Z'))
    }

    #[test]
    fn cb_pz_bound_wrap() {
        let p = Position(25);
        let c = Character('B');
        assert!(c + p == Character('A'))
    }
}

#[cfg(test)]
mod tests_position {

    use super::Position;

    #[test]
    fn construct_0_25() {
        (0..=25).into_iter().for_each(|n| {
            Position::try_from(n).unwrap();
        })
    }

    #[test]
    fn too_large() {
        match Position::try_from(26) {
            Ok(_) => panic!("Should not be able to construct position larger than 25"),
            Err(_) => (),
        }
    }

    #[test]
    fn add() {
        (0..25).into_iter().for_each(|n| {
            let p = Position(n);
            let r = p + 1;
            assert!(r == Position(n + 1))
        })
    }

    #[test]
    fn add_wrap() {
        let p = Position::try_from(25).unwrap();
        let r = p + 1;
        assert!(r == Position(0))
    }
}
