use std::fs;

struct NumberPair (i32, i32);

fn main() {
    let sum_to_find = 2020;
    let numbers = get_numbers("data/input.txt");

    println!("With numbers:\n{}", numbers[0]);

    let pair = find_sum_parts(numbers, sum_to_find);

    println!("Numbers {} and {} added up to the sum {}", pair.0, pair.1, sum_to_find);

    println!("The multiplied value is {}", pair.0 * pair.1);
}

fn get_numbers(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let numbers = contents.lines().map(|text_number| text_number.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    numbers
}

fn find_sum_parts(numbers: Vec<i32>, sum: i32) -> NumberPair {
    /* TODO: Implement this function */

    for () {
        let number_to_look_for = sum - number

        if (is_in_list(number_to_look_for, numbers) {
            return NumberPair(number, number_to_look_for)
        }
    }

    NumberPair(1, 2)
}

fn is_in_list(number: i32, list: Vec<i32>) -> bool {
    if number < 0 {
        // Only positive values in the list
        return false;
    }
    list.iter().any(|&current| current==number)
}
