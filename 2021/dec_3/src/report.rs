use std::fmt;
use std::str::FromStr;
use std::str::Lines;

#[derive(Debug, Clone)]
pub struct ParseDiagnosticDataError;
impl fmt::Display for ParseDiagnosticDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse diagnostic data")
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticData {
    pub gamma_rate: u32,
    pub epsilon_rate: u32,
}

pub trait GetPower {
    fn get_power(&self) -> u32;
}

impl GetPower for DiagnosticData {
    fn get_power(&self) -> u32 {
        self.gamma_rate * self.epsilon_rate
    }
}

impl FromStr for DiagnosticData {
    type Err = ParseDiagnosticDataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char_to_count = '1';
        let alternative_char = '0';
        let character_count = count_char(char_to_count, s.lines());
        let gamma_string = character_count
            .count
            .iter()
            .map(|count| {
                get_most_common_bit(
                    char_to_count,
                    *count,
                    alternative_char,
                    character_count.length,
                )
            })
            .collect::<String>();

        let epsilon_string = character_count
            .count
            .iter()
            .map(|count| {
                get_least_common_bit(
                    char_to_count,
                    *count,
                    alternative_char,
                    character_count.length,
                )
            })
            .collect::<String>();

        let result = DiagnosticData {
            gamma_rate: u32::from_str_radix(&gamma_string, 2).unwrap(),
            epsilon_rate: u32::from_str_radix(&epsilon_string, 2).unwrap(),
        };
        Ok(result)
    }
}

struct CountCharResult {
    count: Vec<u32>,
    length: u32,
}

fn count_char(character: char, iter: Lines<'_>) -> CountCharResult {
    let data_length = 12;
    let mut result = CountCharResult {
        count: vec![0; data_length],
        length: 1000,
    };

    for text_line in iter {
        for (index, c) in text_line.chars().enumerate() {
            if c == character {
                result.count[index] = result.count[index] + 1;
            }
        }
    }
    result
}

fn get_most_common_bit(character: char, count: u32, alternative_char: char, length: u32) -> char {
    if count > length / 2 {
        return character;
    } else if count < length / 2 {
        return alternative_char;
    } else {
        panic!("Impossible to decide most common char!");
    }
}

fn get_least_common_bit(character: char, count: u32, alternative_char: char, length: u32) -> char {
    get_most_common_bit(alternative_char, count, character, length)
}
