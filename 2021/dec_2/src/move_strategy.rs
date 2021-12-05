use crate::submarine::SubmarineCommand;
use crate::submarine::SubmarinePosition;


pub trait MoveStrategy {
    fn execute(
        &self,
        start_pos: SubmarinePosition,
        command: SubmarineCommand,
    ) -> SubmarinePosition;
}

pub struct SimpleMoveStrategy;

impl MoveStrategy for SimpleMoveStrategy {
    fn execute(
        &self,
        start_pos: SubmarinePosition,
        command: SubmarineCommand,
    ) -> SubmarinePosition {
        let  mut position = start_pos.clone();
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
        position
    }
}

pub struct AdvancedMoveStrategy;

impl MoveStrategy for AdvancedMoveStrategy {
    fn execute(
        &self,
        start_pos: SubmarinePosition,
        command: SubmarineCommand,
    ) -> SubmarinePosition {
        let mut position = start_pos.clone();
        match command {
            SubmarineCommand::Forward(distance) => {
                position.horizontal_position = position.horizontal_position + distance;
                position.depth = position.depth + position.aim * distance;
            },
            SubmarineCommand::Up(distance) => {
                position.aim = position.aim - distance;
            },
            SubmarineCommand::Down(distance) => {
                position.aim = position.aim + distance;
            },
        }
        position
    }
}
