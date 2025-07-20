use itertools::Itertools;

pub fn part1(input: &str) -> String {
    let mut jumps = input
        .split_whitespace()
        .map(|number| number.parse::<isize>().expect("Not a number"))
        .collect_vec();

    let mut position = 0isize;

    for counter in 0.. {
        let Some(jump) = jumps.get_mut(position as usize) else {
            return counter.to_string();
        };

        position += *jump;
        *jump += 1;
    }

    unreachable!()
}

pub fn part2(input: &str) -> String {
    let mut jumps = input
        .split_whitespace()
        .map(|number| number.parse::<isize>().expect("Not a number"))
        .collect_vec();

    let mut position = 0isize;

    for counter in 0.. {
        let Some(jump) = jumps.get_mut(position as usize) else {
            return counter.to_string();
        };

        position += *jump;
        *jump += if *jump >= 3 { -1 } else { 1 };
    }

    unreachable!()
}

pub fn run(input: &str) -> (String, String) {
    (part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "0 3 0 1 -3";

        assert_eq!(part1(input), "5");
    }

    #[test]
    fn test_part2() {
        let input = "0 3 0 1 -3";

        assert_eq!(part2(input), "10");
    }
}
