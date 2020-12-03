use std::fs;

const TREE_SYMBOL: char = '#';

#[derive(Debug, Copy, Clone)]
struct Slope {
    right: u64,
    down: u64,
}

#[derive(Debug, Copy, Clone)]
struct Position (u64, u64);

fn main() {
    let tree_map = read_file("data/input.txt");

    let start_position = Position(1,1);
    let slope = Slope {
        right: 3,
        down: 1
    };

    let last_row_index = tree_map.lines().count() as u64;
    let mut trees_hit = 0;
    let mut current_position: Position;

    for i in 1..last_row_index/slope.down {  // Is this working if slope.down is not 1?
        current_position = get_position(start_position, slope, i);
        if is_tree(current_position, &tree_map) {
            trees_hit += 1;
        }
    }

    println!("Hit {} trees on the way down", trees_hit);
}


fn read_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    contents
}

fn is_tree(position: Position, tree_map: &str) -> bool {
    let row = tree_map.lines().nth((position.1 - 1) as usize).expect("Could not find row of map");
    let map_width = row.chars().count() as u64;
    let translated_x = (position.0 -1) % (map_width);

    let char_row: Vec<char> = row.chars().collect();
    let result = char_row[(translated_x) as usize] == TREE_SYMBOL;
    result
}

fn get_position(start_position: Position, slope: Slope, runs: u64) -> Position {
    Position(start_position.0 + slope.right * runs, start_position.1 + slope.down * runs)
}
