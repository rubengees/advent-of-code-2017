use std::cmp::max;
use std::collections::HashMap;

pub fn part1(input: &str) -> String {
    let number: i32 = input.parse().expect("Not a number");

    let size = (number as f64).sqrt().ceil() as i32;
    let center = (((size - 1) as f64) / 2.0).ceil() as i32;

    max(0, center - 1 + (center - number % size).abs()).to_string()
}

pub fn part2(input: &str) -> String {
    let number: i32 = input.parse().expect("Not a number");

    // There is probably a better data structure for this.
    let mut spiral = HashMap::new();
    let mut x = 0;
    let mut y = 0;

    spiral.insert((0, 0), 1);

    let result = loop {
        let right = *spiral.get(&(x + 1, y)).unwrap_or(&0);
        let top = *spiral.get(&(x, y - 1)).unwrap_or(&0);
        let left = *spiral.get(&(x - 1, y)).unwrap_or(&0);
        let bottom = *spiral.get(&(x, y + 1)).unwrap_or(&0);

        // Decide in which direction we need to go by checking the options counterclockwise.
        let (x_dir, y_dir) = if bottom == 0 && right != 0 {
            (0, 1) // Go down
        } else if left == 0 && bottom != 0 {
            (-1, 0) // Go left
        } else if top == 0 && left != 0 {
            (0, -1) // Go up
        } else {
            (1, 0) // Go right
        };

        x += x_dir;
        y += y_dir;

        // Find all neighbours of the new position and sum them up.
        let next_value = [-1, 0, 1]
            .iter()
            .flat_map(|nx| [-1, 0, 1].map(|ny| spiral.get(&(x + nx, y + ny)).unwrap_or(&0)))
            .sum();

        // If the next value is greater than the input number, we break and return it.
        if next_value > number {
            break next_value;
        }

        spiral.insert((x, y), next_value);
    };

    result.to_string()
}

pub fn run(input: &str) -> (String, String) {
    (part1(input), part2(input))
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

    #[test]
    fn test_part2_1() {
        assert_eq!(part2("1"), "2");
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2("2"), "4");
    }

    #[test]
    fn test_part2_3() {
        assert_eq!(part2("25"), "26");
    }

    #[test]
    fn test_part2_4() {
        assert_eq!(part2("304"), "330");
    }

    #[test]
    fn test_part2_5() {
        assert_eq!(part2("747"), "806");
    }
}
