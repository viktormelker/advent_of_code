use std::fs;
use structopt::StructOpt;

mod move_strategy;
use crate::move_strategy::*;

mod submarine;
use crate::submarine::*;

#[derive(StructOpt)]
struct Cli {
    // The version (1 or 2)
    version: u8,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() {
    let args = Cli::from_args();
    let input = read_file(args.path);
    let move_strategy: MoveStrategy;

    match args.version {
        1 => move_strategy = MoveStrategy::Simple(SimpleMoveStrategy),
        2 => move_strategy = MoveStrategy::Advanced(AdvancedMoveStrategy),
        _ => panic!("Invalid version"),
    }

    let commands: Vec<SubmarineCommand> = input
        .lines()
        .map(|line| line.parse::<SubmarineCommand>().unwrap())
        .collect();
    let start_position: SubmarinePosition = SubmarinePosition {
        horizontal_position: 0,
        depth: 0,
        aim: 0,
    };

    let position = simulate(start_position, commands, move_strategy);

    println!(
        "Ended up on position: {}, depth: {}",
        position.horizontal_position, position.depth
    )
}

fn simulate(
    start_position: SubmarinePosition,
    commands: Vec<SubmarineCommand>,
    move_strategy: MoveStrategy,
) -> SubmarinePosition {
    let mut position = start_position.clone();

    for command in commands {
        position = move_strategy.execute(position, command);
    }

    position
}

fn read_file(path: std::path::PathBuf) -> String {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    contents
}
