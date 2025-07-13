use std::{env, fs, process};

mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: i32 = args
        .get(1)
        .map(|it| it.parse().ok())
        .flatten()
        .expect("Please provide a day number as an argument");

    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let contents = contents.trim();

    let [part1, part2] = match day {
        1 => [day1::part1(contents), day1::part2(contents)],
        2 => [day2::part1(contents), "Not implemented".into()],
        _ => {
            eprintln!("Day {} is not implemented yet", day);
            process::exit(1);
        }
    };

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
