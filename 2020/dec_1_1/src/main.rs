use std::fs;

struct NumberPair {
    first: i32,
    second: i32,
}

fn main() {
    let sum_to_find = 2020;
    let numbers = get_numbers("data/input.txt");

    println!("With numbers:\n{}", numbers[0]);

    let pair = find_sum_parts(numbers, sum_to_find);

    println!("Numbers {} and {} added up to the sum {}", pair.first, pair.second, sum_to_find);

    println!("The multiplied value is {}", pair.first * pair.second);
}

fn get_numbers(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let numbers = contents.lines().map(|text_number| text_number.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    numbers
}

fn find_sum_parts(numbers: Vec<i32>, sum: i32) -> NumberPair {
    /* TODO: Implement this function */

    let result = NumberPair {
        first: 1,
        second: 2,
    };
    result
}
