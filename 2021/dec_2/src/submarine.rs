use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct SubmarinePosition {
    pub horizontal_position: u32,
    pub depth: u32,
    pub aim: u32,
}

#[derive(Debug, Clone)]
pub struct ParseEnumError;
impl fmt::Display for ParseEnumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse Enum")
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SubmarineCommand {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for SubmarineCommand {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(' ').collect();
        if words.len() != 2 {
            return Err(ParseEnumError);
        }
        let word_tuple = (words[0], words[1]);

        match word_tuple {
            ("forward", distance) => {
                return Ok(SubmarineCommand::Forward(distance.parse::<u32>().unwrap()));
            }
            ("up", distance) => {
                return Ok(SubmarineCommand::Up(distance.parse::<u32>().unwrap()));
            }
            ("down", distance) => {
                return Ok(SubmarineCommand::Down(distance.parse::<u32>().unwrap()));
            }
            _ => return Err(ParseEnumError),
        }
    }
}
