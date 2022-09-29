#[derive(Debug)]
pub struct Character(u8);

impl TryFrom<char> for Character {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let value_uppercase = value.to_ascii_uppercase();
        match value_uppercase{
            'A'..='Z' => Ok(Character(value_uppercase as u8 - b'A')),
            _ => Err("Parsing error: only valid characters are A-Z")
        }
    }
}
