use itertools::Itertools;
use std::collections::HashSet;

pub fn part1(input: &str) -> String {
    let mut banks = input
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("Not a number"))
        .collect_vec();

    cycle_until_seen(&mut banks).to_string()
}

fn cycle_until_seen(banks: &mut [u32]) -> i32 {
    let mut seen = HashSet::from([hash(banks)]);

    for counter in 1.. {
        redistribute(banks);

        if !seen.insert(hash(banks)) {
            return counter;
        }
    }

    unreachable!()
}

fn hash(banks: &[u32]) -> String {
    banks.iter().join(" ")
}

fn redistribute(banks: &mut [u32]) {
    let Some(next_index) = max_position(banks) else {
        return;
    };

    let current = banks[next_index];
    let len = banks.len() as u32;

    banks[next_index] = 0;

    let to_add = current / len;
    let remaining = current % len;

    // Evenly distribute the blocks among all banks.
    for value in banks.iter_mut() {
        *value += to_add;
    }

    // Use up the remaining blocks (if currently did not distribute evenly).
    for i in 0..remaining {
        let idx = (next_index + 1 + i as usize) % banks.len();
        banks[idx] += 1;
    }
}

// Returns the index of the maximum element in the slice, or None if the slice is empty.
// If there are multiple maximum elements, the first occurrence is returned.
//
// We can't use position_max since it returns the last occurrence of the maximum element.
fn max_position<T: PartialOrd>(slice: &[T]) -> Option<usize> {
    if slice.is_empty() {
        return None;
    }

    let mut max_pos = 0;
    for (i, item) in slice.iter().enumerate().skip(1) {
        if *item > slice[max_pos] {
            max_pos = i;
        }
    }

    Some(max_pos)
}

pub fn part2(input: &str) -> String {
    let mut banks = input
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("Not a number"))
        .collect_vec();

    cycle_until_seen(&mut banks);
    cycle_until_seen(&mut banks).to_string()
}

pub fn run(input: &str) -> (String, String) {
    (part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "0 2 7 0";

        assert_eq!(part1(input), "5");
    }

    #[test]
    fn test_redistribute_1() {
        let mut banks = vec![0, 2, 7, 0];
        redistribute(&mut banks);
        assert_eq!(banks, vec![2, 4, 1, 2]);
    }

    #[test]
    fn test_redistribute_2() {
        let mut banks = vec![2, 4, 1, 2];
        redistribute(&mut banks);
        assert_eq!(banks, vec![3, 1, 2, 3]);
    }

    #[test]
    fn test_redistribute_3() {
        let mut banks = vec![3, 3, 3, 3];
        redistribute(&mut banks);
        assert_eq!(banks, vec![0, 4, 4, 4]);
    }

    #[test]
    fn test_part2() {
        let input = "2 4 1 2";

        assert_eq!(part2(input), "4");
    }
}
