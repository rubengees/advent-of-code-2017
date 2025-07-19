use std::cmp::max;

pub fn part1(input: &str) -> String {
    let number: i32 = input.parse().expect("Not a number");

    let size = (number as f64).sqrt().ceil() as i32;
    let center = (((size - 1) as f64) / 2.0).ceil() as i32;

    max(0, center - 1 + (center - number % size).abs()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1("1"), "0");
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1("12"), "3");
    }

    #[test]
    fn test_part1_3() {
        assert_eq!(part1("23"), "2");
    }

    #[test]
    fn test_part1_4() {
        assert_eq!(part1("1024"), "31");
    }
}
