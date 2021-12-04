use std::fs;

struct SonarSweepResult {
    num_decreased: u32,
    num_increased: u32,
}

fn main() {
    let input = read_file("./data/input.txt");
    let numbers: Vec<u32> = input.lines().map(|line| line.parse::<u32>().unwrap()).collect();
    let sonar_analysis = analyze_sonar_sweep(numbers);
    println!("Depth increased {} times and decreased {} times", sonar_analysis.num_increased, sonar_analysis.num_decreased)
}


fn analyze_sonar_sweep(numbers: Vec<u32>) -> SonarSweepResult {
    let mut num_increased: u32 = 0;
    let mut num_decreased: u32 = 0;

    let mut previous_number = 0;
    let mut first_measurement: bool = true;

    for number in numbers {

        if first_measurement {
            first_measurement = false
        }
        else if number > previous_number {
            num_increased = num_increased + 1;
        }
        else if number <  previous_number {
            num_decreased = num_decreased + 1;
        }
        previous_number = number;
    }

    SonarSweepResult {num_decreased: num_decreased, num_increased: num_increased}
}

fn read_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}
