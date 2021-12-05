use crate::submarine::*;

pub enum MoveStrategy {
    Simple(SimpleMoveStrategy),
    Advanced(AdvancedMoveStrategy),
}

impl MoveSubmarine for MoveStrategy {
    fn execute(&self, start_pos: SubmarinePosition, command: SubmarineCommand) -> SubmarinePosition {
        use MoveStrategy::*;
        match self {
            Simple(strategy) => strategy.execute(start_pos, command),
            Advanced(strategy) => strategy.execute(start_pos, command),
        }
    }
}

pub trait MoveSubmarine {
    fn execute(
        &self,
        start_pos: SubmarinePosition,
        command: SubmarineCommand,
    ) -> SubmarinePosition;
}

pub struct SimpleMoveStrategy;

impl MoveSubmarine for SimpleMoveStrategy {
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

impl MoveSubmarine for AdvancedMoveStrategy {
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
