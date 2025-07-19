use itertools::Itertools;

pub fn part1(input: &str) -> String {
    let digits = input
        .chars()
        .map(|char| char.to_digit(10).expect("Not a number"))
        .collect_vec();

    let mut sum = 0;

    for (i, digit) in digits.iter().enumerate() {
        let next_digit = digits
            .get((i + 1) % digits.len())
            .expect("Failed to get next digit");

        if digit == next_digit {
            sum += digit;
        }
    }

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let digits = input
        .chars()
        .map(|char| char.to_digit(10).expect("Not a number"))
        .collect_vec();

    let mut sum = 0;

    for (i, digit) in digits.iter().enumerate() {
        let next_digit = digits
            .get((i + (digits.len() / 2)) % digits.len())
            .expect("Failed to get next digit");

        if digit == next_digit {
            sum += digit;
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1("1122"), "3");
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1("1111"), "4");
    }

    #[test]
    fn test_part1_3() {
        assert_eq!(part1("1234"), "0");
    }

    #[test]
    fn test_part1_4() {
        assert_eq!(part1("91212129"), "9");
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(part2("1212"), "6");
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2("1221"), "0");
    }

    #[test]
    fn test_part2_3() {
        assert_eq!(part2("123425"), "4");
    }

    #[test]
    fn test_part2_4() {
        assert_eq!(part2("123123"), "12");
    }

    #[test]
    fn test_part2_5() {
        assert_eq!(part2("12131415"), "4");
    }
}
