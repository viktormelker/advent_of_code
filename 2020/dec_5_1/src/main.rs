use std::fs;

const ROW_INDICATOR_LENGTH: usize = 7;
const COLUMN_INDICATOR_LENGTH: usize = 3;
const SEATS_PER_ROW: u64 = 8;

const MAX_ROWS: u64 = 127;
const MAX_COLUMNS: u64 = SEATS_PER_ROW;
const UPPER_HALF_CHARS: [char; 2] = ['B', 'R'];

enum BinaryIndicator {
    UPPER,
    LOWER
}

struct BoardingPass {
    row_indicator: String,
    column_indicator: String,
}

impl BoardingPass {
    fn get_row(&self) -> u64 {
        let min: u64 = 0;
        let max: u64 = MAX_ROWS;
        let binary_indicators = self.row_indicator
            .chars()
            .map(
                |letter| get_binary_indicator(&letter))
            .collect();
        binary_search(min, max, binary_indicators)
    }

    fn get_column(&self) -> u64 {
        let min: u64 = 0;
        let max: u64 = MAX_COLUMNS;
        let binary_indicators = self.column_indicator
            .chars()
            .map(
                |letter| get_binary_indicator(&letter))
            .collect();
        binary_search(min, max, binary_indicators)
    }

    fn get_seat_id(&self) -> u64 {
        SEATS_PER_ROW * self.get_row() + self.get_column()
    }

    fn print(&self) {
        println!(
            "BoardingPass({}, {}): row {}, column {}, seat_id {}",
            self.row_indicator,
            self.column_indicator,
            self.get_row(),
            self.get_column(),
            self.get_seat_id()
        );
    }
}

fn main() {
    let bp_input = read_file("./data/input.txt");
    let boarding_passes = parse_bps(&bp_input);

    let mut max_seat_id = 0;

    for boarding_pass in boarding_passes.iter() {
        let seat_id = boarding_pass.get_seat_id();
        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
        boarding_pass.print();
    }

    println!("Highest seat_id found was {}", max_seat_id);
}

fn parse_bps(bp_input: &String) -> Vec<BoardingPass> {
    let mut boarding_passes: Vec<BoardingPass> = vec![];
    for bp_string in bp_input.lines() {
        let row_indicator = bp_string
            .get(..ROW_INDICATOR_LENGTH)
            .unwrap();
        let column_indicator = bp_string
            .get(ROW_INDICATOR_LENGTH..ROW_INDICATOR_LENGTH + COLUMN_INDICATOR_LENGTH)
            .unwrap();

        boarding_passes.push(BoardingPass {
            row_indicator: String::from(row_indicator),
            column_indicator: String::from(column_indicator),
        })
    }
    boarding_passes
}

fn read_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

fn binary_search(min: u64, max: u64, binary_indicators: Vec<BinaryIndicator>) -> u64 {
    let mut min = min;
    let mut max = max;
    for indicator in binary_indicators.iter() {
        match indicator {
            BinaryIndicator::UPPER => {
                min = (max + 1 + min) / 2;
            }
            BinaryIndicator::LOWER => {
                max = (max + 1 + min) / 2 - 1;
            }
        }
        if max == min {
            return max
        }
    }
    0  // This should never happen
}

fn get_binary_indicator(letter: &char) -> BinaryIndicator {
    if UPPER_HALF_CHARS.contains(letter) {
        return BinaryIndicator::UPPER;
    }
    else {
        BinaryIndicator::LOWER
    }
}
