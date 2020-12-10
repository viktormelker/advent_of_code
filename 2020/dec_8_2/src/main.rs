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

#[derive(Debug, Copy, Clone, PartialEq)]
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
    let input = read_file("./data/input.txt");
    let instructions: Vec<Instruction> = input.lines().map(|line| line.parse::<Instruction>().unwrap()).collect();
    let potential_switch_indices: Vec<bool> = instructions.iter().map(|instruction| instruction.identifier != Operation::ACCUMULATE).collect();
    let mut result: (bool, i64) = (true, 0);
    let mut temp_instructions: Vec<Instruction>;

    for (i, switch) in potential_switch_indices.iter().enumerate() {
        if *switch {
            temp_instructions = instructions.clone();
            println!("Attempting to switch instruction on index {}", i);
            let bad_instruction: Instruction = temp_instructions.remove(i);
            let good_instruction: Instruction = Instruction {
                identifier: {
                    if bad_instruction.identifier == Operation::JUMP {
                        Operation::NOP
                    } else {
                        Operation::NOP
                    }
                },
                ..bad_instruction
            };
            temp_instructions.insert(i, good_instruction);
            println!("Length of instructions to check {}", temp_instructions.len());
            result = run(& mut temp_instructions);
            if !result.0 {
                println!("Found faulty statement on index {}", i);
                break;
            }
            println!("------------ Finished for this time ----------- ");
        }
    }

    println!("Accumulated value was {}!", result.1);
    println!("loop was infinite={}!", result.0);
}

fn run(instructions: &mut Vec<Instruction>) -> (bool, i64) {
    let mut current_pos: usize = 0;
    let mut sum: i64 = 0;
    let mut instruction: Instruction;
    let mut infinite: bool = false;

    loop {
        instruction = instructions.remove(current_pos);
        if instruction.visited {
            println!("Visited instruction {} for the second time", current_pos);
            infinite = true;
            break;
        }
        else {
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

        if current_pos >= instructions.len() {
            println!("Finished without finding infinite loops!");
            return (false, sum);
        }
    }

    (infinite, sum)
}


fn read_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}
