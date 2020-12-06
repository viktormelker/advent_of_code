use std::fs;

const ROW_INDICATOR_LENGTH: usize = 7;
const COLUMN_INDICATOR_LENGTH: usize = 3;
const SEATS_PER_ROW: u64 = 8;

const MAX_ROWS: u64 = 127;
const MAX_COLUMNS: u64 = SEATS_PER_ROW - 1;
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
}

fn main() {
    let bp_input = read_file("./data/input.txt");
    let boarding_passes = parse_bps(&bp_input);

    let mut seat_ids: Vec<u64> = boarding_passes.iter().map(|boarding_pass| boarding_pass.get_seat_id()).collect();
    seat_ids.sort();
    println!("Found {} boarding passes", seat_ids.iter().count());

    let missing_seat_ids = get_missing_seat_ids(seat_ids);
    println!("Found {} missing boarding passes", missing_seat_ids.iter().count());
    println!("Found missing boarding passes {:?}", missing_seat_ids);
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

fn get_missing_seat_ids(seat_ids: Vec<u64>) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];
    for seat_id in *seat_ids.first().unwrap()..*seat_ids.last().unwrap() {
        if !seat_ids.contains(&seat_id) & has_all_neighbours(seat_id, seat_ids.clone()) {
            result.push(seat_id);
        }
    }
    result
}

fn has_all_neighbours(seat_id: u64, seat_ids: Vec<u64>) -> bool {
    if seat_id > 0 {
        if seat_ids.contains(&(seat_id-1)) {
            if seat_ids.contains(&(seat_id+1)) {
                return true
            }
        }
    }
    false
}
