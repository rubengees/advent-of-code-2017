use std::{env, fs, process};

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = args
        .get(1)
        .expect("Please provide a day number as an argument")
        .parse::<i32>()
        .expect("Must be a number");

    let contents = fs::read_to_string("input.txt").expect("Failed to read input.txt");

    let contents = contents.trim();

    let (part1, part2) = match day {
        1 => days::day1::run(contents),
        2 => days::day2::run(contents),
        3 => days::day3::run(contents),
        4 => days::day4::run(contents),
        _ => {
            eprintln!("Day {} is not implemented yet", day);
            process::exit(1);
        }
    };

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
