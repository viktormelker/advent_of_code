use std::fs;
use structopt::StructOpt;

mod move_strategy;
use crate::move_strategy::AdvancedMoveStrategy;
use crate::move_strategy::MoveStrategy;

mod submarine;
use crate::submarine::SubmarineCommand;
use crate::submarine::SubmarinePosition;

#[derive(StructOpt)]
struct Cli {
    // The version (1 or 2)
    version: u8,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

/*struct Context<S> {
    strategy: S,
}

impl<S> Context<S>
where
    S: MoveStrategy,
{
    fn move_sub(
        &self,
        start_pos: SubmarinePosition,
        command: SubmarineCommand,
    ) -> SubmarinePosition {
        self.strategy.execute(start_pos, command)
    }
}*/


fn main() {
    let args = Cli::from_args();
    let input = read_file(args.path);
    let move_strategy;

    match args.version {
        1 => move_strategy = AdvancedMoveStrategy,
        2 => move_strategy = AdvancedMoveStrategy,
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

    let position = simulate(start_position, commands, &move_strategy);

    println!(
        "Ended up on position: {}, depth: {}",
        position.horizontal_position, position.depth
    )
}

fn simulate(
    start_position: SubmarinePosition,
    commands: Vec<SubmarineCommand>,
    move_strategy: &impl MoveStrategy,
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
