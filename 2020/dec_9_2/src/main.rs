use std::fs;

const NUM_TO_FIND: u64 = 1398413738;

fn main() {
    let input = read_file("./data/input.txt");
    let numbers: Vec<u64> = input.lines().map(|line| line.parse::<u64>().unwrap()).collect();
    let mut current_numbers: Vec<u64>;

    for (i, _) in numbers.iter().enumerate() {
        current_numbers = vec![];
        println!("Looking for a solution starting on position {}", i);

        if adds_up(&mut current_numbers, i, &numbers) {
            println!("Found the right place");
            break;
        }
    }
}


fn read_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

fn adds_up(current_numbers: &mut Vec<u64>, index: usize, numbers: &Vec<u64>) -> bool {
    let next_number = numbers[index];
    let sum: u64 = current_numbers.iter().sum::<u64>() + next_number;
    // println!("searching with numbers {:?}", current_numbers);
    current_numbers.push(next_number);
    if sum == NUM_TO_FIND {
        println!("Found the numbers that add up {:?}", current_numbers);
        println!(
            "The sum of min and max in array is {}",
            current_numbers.iter().min().unwrap() + current_numbers.iter().max().unwrap()
        );
        return true;
    }
    else if sum > NUM_TO_FIND {
        return false;
    }
    else {
        return adds_up(current_numbers, index + 1, numbers);
    }
}
