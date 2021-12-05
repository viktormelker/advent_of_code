use std::fs;
use structopt::StructOpt;

mod report;
use crate::report::*;


#[derive(StructOpt)]
struct Cli {
    // The version (1 or 2)
    version: u8,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() {
    let args = Cli::from_args();
    let input = read_file(args.path);

    match args.version {
        1 => println!("Going for my first star!"),
        _ => panic!("Invalid version"),
    }
    let diagnostic_data = input.parse::<DiagnosticData>().unwrap();

    println!("Submarine has power consumption {}", diagnostic_data.get_power())
}


fn read_file(path: std::path::PathBuf) -> String {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    contents
}
