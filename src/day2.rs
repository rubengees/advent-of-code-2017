use itertools::Itertools;

pub fn part1(input: &str) -> String {
    let sum: i32 = input
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|num| num.parse().expect("Not a number"))
                .collect();

            let min = numbers.iter().min().unwrap_or(&0);
            let max = numbers.iter().max().unwrap_or(&0);

            max - min
        })
        .sum();

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let sum: i32 = input
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|num| num.parse().expect("Not a number"))
                .collect();

            numbers
                .iter()
                .permutations(2)
                .find_map(|pair| match pair.as_slice() {
                    [a, b] if **b != 0 && *a % *b == 0 => Some(*a / *b),
                    _ => None,
                })
                .unwrap_or(0)
        })
        .sum();

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "5 1 9 5\n7 5 3\n2 4 6 8";

        assert_eq!(part1(input), "18");
    }

    #[test]
    fn test_part2() {
        let input = "5 9 2 8\n9 4 7 3\n3 8 6 5";

        assert_eq!(part2(input), "9");
    }
}
