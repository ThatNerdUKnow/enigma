use std::{
    fmt::{Display, Formatter},
    ops::Add,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Character(char);

#[derive(PartialEq, Eq, Clone, Copy)]
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

impl TryFrom<char> for Position {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let v = value.to_ascii_uppercase();
        let offset = v as u8 - b'A';
        Position::try_from(offset)
    }
}

impl Add<u8> for Position {
    type Output = Position;

    fn add(self, rhs: u8) -> Self::Output {
        let offset = (self.0 + rhs) % 26;
        Position(offset)
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
