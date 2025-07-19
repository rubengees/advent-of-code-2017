use crate::days::run_day;
use std::{env, fs};

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

    let (part1, part2) =
        run_day(day, contents).expect(format!("Day {day} is not implemented yet").as_str());

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
