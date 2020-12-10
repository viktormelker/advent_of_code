use std::fmt;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct ParseEnumError;
impl fmt::Display for ParseEnumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse Enum")
    }
}

#[derive(Debug, Clone)]
struct ParseStructError;
impl fmt::Display for ParseStructError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse struct")
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    NOP,
    ACCUMULATE,
    JUMP,
}

impl FromStr for Operation {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "nop" {
            return Ok(Operation::NOP);
        } else if s == "acc" {
            return Ok(Operation::ACCUMULATE);
        } else if s == "jmp" {
            return Ok(Operation::JUMP);
        } else
        {
            Err(ParseEnumError)
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Argument {
    multiplier: i64,
    value: i64,
}
impl FromStr for Argument {
    type Err = ParseStructError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sign = s.chars().next().unwrap();
        let multiplier = match sign {
            '-' => -1,
            '+' => 1,
            _ => return Result::Err(ParseStructError),
        };
        let value = &s[1..].parse::<i64>().unwrap();

        Ok(Argument{
            multiplier: multiplier,
            value: *value,
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    identifier: Operation,
    argument: Argument,
    visited: bool,
}

impl FromStr for Instruction {
    type Err = ParseStructError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut instruction_parts = line.split_whitespace();
        let identifier = instruction_parts.next().unwrap();
        let argument_str = instruction_parts.next().unwrap();

        Ok(
            Instruction{
                identifier: identifier.parse::<Operation>().unwrap(),
                argument: argument_str.parse::<Argument>().unwrap(),
                visited: false
            }
        )
    }
}

fn main() {
    let input = read_file("./data/test_input.txt");
    let mut instructions: Vec<Instruction> = input.lines().map(|line| line.parse::<Instruction>().unwrap()).collect();
    let mut current_pos: usize = 0;
    let mut sum: i64 = 0;
    let mut instruction: Instruction;

    loop {
        instruction = instructions.remove(current_pos);
        if instruction.visited  || sum > 100 {
            println!("Visited instruction {} for the second time", current_pos);
            break;
        }
        else {
            println!("Visited instruction at position {}", current_pos);
            instruction.visited = true;
        }
        instructions.insert(current_pos, instruction);

        match instruction.identifier {
            Operation::ACCUMULATE => {
                sum += instruction.argument.value * instruction.argument.multiplier;
                current_pos += 1;
            },
            Operation::JUMP => {
                current_pos = (current_pos as i64 + instruction.argument.value * instruction.argument.multiplier) as usize;
            }
            Operation::NOP => {
                current_pos += 1;
            }
        }
    }

    println!("Accumulated value was {}!", sum);
}


fn read_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}
