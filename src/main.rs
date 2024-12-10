use advent_of_code_2024::*;
use clap::{Arg, Command};

fn main() {
    // Example Command:
    // cargo run -- day1 --file inputs/input.txt
    let matches = Command::new("Day Runner")
        .about("Run day-specific code with an optional file path")
        .arg(
            Arg::new("day")
                .help("The day to run (e.g., day1, day2, day3)")
                .required(true),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .help("Optional file path")
                .value_name("FILE"), // Optional value with a name for clarity
        )
        .get_matches();

    let day = matches
        .get_one::<String>("day")
        .expect("Day argument is required");

    let file_path = matches
        .get_one::<String>("file")
        .expect("Input is required");

    let _result = match day.as_str() {
        "day1" => day1::execute(file_path),
        "day2" => day2::execute(file_path),
        "day3" => day3::execute(file_path),
        "day4" => day4::execute(file_path),
        "day5" => day5::execute(file_path),
        "day6" => day6::execute(file_path),
        "day7" => day7::execute(file_path),
        "day8" => day8::execute(file_path),
        "day9" => day9::execute(file_path),
        _ => {
            eprintln!("Unsupported day: {}", day);
            std::process::exit(1);
        }
    };
}
