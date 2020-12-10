use std::fs;
use std::collections::LinkedList;

fn main() {
    let input = read_file("./data/input.txt");
    let numbers: Vec<u64> = input.lines().map(|line| line.parse::<u64>().unwrap()).collect();
    let mut passed_numbers: LinkedList<u64> =  LinkedList::new();

    for number in numbers {
        if passed_numbers.len() < 25 {
            passed_numbers.push_front(number);
        }
        else {
            if !is_valid(number, &passed_numbers) {
                println!("Found invalid number {}", number);
                break;
            }
            passed_numbers.push_front(number);
            passed_numbers.pop_back().unwrap();
        }
    }
}


fn is_valid(number: u64, list: &LinkedList<u64>) -> bool {
    for num1 in list.iter() {
        if *num1 > number {
            continue
        }
        else {
            let num2: u64 = number - num1;
            if list.contains(&num2) {
                return true;
            }
        }
    }
    false
}


fn read_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}
