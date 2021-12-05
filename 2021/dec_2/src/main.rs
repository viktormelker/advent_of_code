use std::fmt;
use std::fs;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    // The version (1 or 2)
    version: u8,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Debug, Clone)]
struct ParseEnumError;
impl fmt::Display for ParseEnumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse Enum")
    }
}

#[derive(Debug, Clone)]
struct SubmarinePosition {
    horizontal_position: u32,
    depth: u32,
    aim: u32,
}

#[derive(Debug, Copy, Clone)]
enum SubmarineCommand {
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

fn main() {
    let args = Cli::from_args();
    let input = read_file(args.path);

    match args.version {
        1 => println!("First star of the challange"),
        _ => panic!("Invalid version"),
    }

    let commands: Vec<SubmarineCommand> = input
        .lines()
        .map(|line| line.parse::<SubmarineCommand>().unwrap())
        .collect();
    let start_position: SubmarinePosition = SubmarinePosition {
        horizontal_position: 0,
        depth: 0,
        aim: 0
    };

    let position = simulate(start_position, commands);

    println!("Ended up on position: {}, depth: {}", position.horizontal_position, position.depth)
}

fn simulate(
    start_position: SubmarinePosition,
    commands: Vec<SubmarineCommand>,
) -> SubmarinePosition {
    let mut position = start_position.clone();

    for command in commands {
        match command {
            SubmarineCommand::Forward(distance) => {
                position.horizontal_position = position.horizontal_position + distance;
            },
            SubmarineCommand::Up(distance) => {
                position.depth = position.depth - distance;
            },
            SubmarineCommand::Down(distance) => {
                position.depth = position.depth + distance;
            },
        }

    }

    position
}

fn read_file(path: std::path::PathBuf) -> String {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    contents
}
