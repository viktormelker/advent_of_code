use std::fs;

#[derive(Debug, Copy, Clone, PartialEq)]
enum SeatState {
    EMPTY,
    OCCUPIED,
    FLOOR,
}

#[derive(Debug, Clone, PartialEq)]
struct Seat {
    state: SeatState,
    row: usize,
    column: usize
}
impl Seat {
    fn next_state(&self, seats: Vec<SeatRow>) -> SeatState {
        let neighbours = self.get_neighbours(&seats);
        let occupied_neighbours = neighbours
            .iter()
            .fold(0, |acc, seat| {
                match seat{
                    Some(seat) => {
                        if seat.state == SeatState::OCCUPIED {
                            return acc + 1;
                        }
                        else { acc }
                    },
                    None => return acc
                }
            });

        match self.state {
            SeatState::EMPTY => {
                if occupied_neighbours == 0 {
                    return SeatState::OCCUPIED
                }
                else { return self.state}
            }
            SeatState::OCCUPIED => {
                if occupied_neighbours >= 4 {
                    return SeatState::EMPTY
                }
                else { return self.state }
            }
            SeatState::FLOOR => return SeatState::FLOOR
        }
    }

    fn from_char(input: char, row: usize, column: usize) -> Seat {
        match input {
            '.' => Seat{state: SeatState::FLOOR, row: row, column: column},
            '#' => Seat{state: SeatState::OCCUPIED, row: row, column: column},
            'L' => Seat{state: SeatState::EMPTY, row: row, column: column},
            _ => panic!("Found invalid seat state")
        }
    }

    fn get_neighbours(&self, seats: &Vec<SeatRow>) -> Vec<Option<&Seat>> {
        let mut result: Vec<Option<&Seat>> =  vec![];

        let rows = vec![
            seats.get(self.row - 1),
            seats.get(self.row),
            seats.get(self.row + 1)
        ];

        for row_i in rows {
            match row_i {
                Some(the_row) => {
                    if the_row.row == self.row {
                        result.push(the_row.seats.get(self.column - 1));
                        result.push(the_row.seats.get(self.column + 1));
                    }
                    else {
                        result.push(the_row.seats.get(self.column - 1));
                        result.push(the_row.seats.get(self.column));
                        result.push(the_row.seats.get(self.column + 1));
                    }
                },
                None => continue
            }
        }
        return result;
    }

    fn count(&self, state: SeatState) -> u64 {
        match self.state {
            state => return 1,
            _ => return 0
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct SeatRow {
    seats: Vec<Seat>,
    row: usize
}

impl SeatRow {
    fn from_string(input: &str, row: usize) -> SeatRow {
        let result: Vec<Seat> = input
            .chars()
            .into_iter()
            .enumerate()
            .map(|(column, c)| Seat::from_char(c, row, column))
            .collect();

        SeatRow {seats: result, row: row}
    }

    fn next_state(&self, seats: Vec<SeatRow>) -> SeatRow {
        SeatRow {
            seats: self.seats
                .iter()
                .map(|seat| Seat{state: seat.next_state(seats), column: seat.column, row: seat.row})
                .collect(),
            row:  self.row
        }
    }

    fn count(&self, state: SeatState) -> u64 {
        self.seats.iter().fold(0, |acc, seat| acc + seat.count(state))
    }
}

fn main() {
    let input = read_file("./data/input.txt");
    let mut previous_rows: Vec<SeatRow> = vec![];
    let mut rows: Vec<SeatRow> = input.lines()
        .enumerate()
        .map(|(row, line)| SeatRow::from_string(line, row))
        .collect();

    println!("Got rows {:?}", rows);

    loop {
        previous_rows = rows;
        rows = rows.iter().map(|row| row.next_state(rows)).collect();

        println!("Updated rows {:?}", rows);

        if rows == previous_rows {
            println!("Detected no change since last iteration. We are done!");
            break;
        }
    }

    let num_occupied = rows
        .iter()
        .fold(0, |acc, row| acc + row.count(SeatState::OCCUPIED));
    println!("The number of occupied seats is {}", num_occupied);
}


fn read_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}
