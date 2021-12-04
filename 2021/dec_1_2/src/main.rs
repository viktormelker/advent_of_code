use structopt::StructOpt;
use std::fs;


#[derive(StructOpt)]
struct Cli {
    // The version (1 or 2)
    version: u8,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}


struct SonarSweepResult {
    num_decreased: u32,
    num_increased: u32,
}

fn main() {
    let args = Cli::from_args();
    let filter_size: u8;
    let input = read_file(args.path);
    let numbers: Vec<u32> = input.lines().map(|line| line.parse::<u32>().unwrap()).collect();

    match args.version {
        1 => filter_size = 1,
        2 => filter_size = 3,
        _ => panic!("Invalid version"),
    }


    let sonar_data = run_smoothing_filter(numbers, filter_size);
    let sonar_analysis = analyze_sonar_sweep(sonar_data);
    println!("Depth increased {} times and decreased {} times", sonar_analysis.num_increased, sonar_analysis.num_decreased)
}


fn run_smoothing_filter(data: Vec<u32>, size: u8) -> Vec<u32> {
    // use slice and windows to get
    let slice = &data[..];
    let mut result: Vec<u32> = vec![];

    for window in slice.windows(size as usize) {
        let sum: u32 = window.iter().sum();
        result.push(sum);
    }
    result
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

fn read_file(path: std::path::PathBuf) -> String {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    contents
}
