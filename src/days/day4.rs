use itertools::Itertools;

pub fn part1(input: &str) -> String {
    let valid_passphrases = input
        .lines()
        .filter(|line| is_passphrase_valid(line))
        .count();

    valid_passphrases.to_string()
}

fn is_passphrase_valid(input: &str) -> bool {
    let words = input.split_whitespace().collect_vec();
    let unique_words = words.iter().unique().collect_vec();

    words.len() == unique_words.len()
}

pub fn part2(input: &str) -> String {
    let valid_passphrases = input
        .lines()
        .filter(|line| is_passphrase_valid_2(line))
        .count();

    valid_passphrases.to_string()
}

fn is_passphrase_valid_2(input: &str) -> bool {
    let words_sorted = input
        .split_whitespace()
        .map(|word| word.chars().sorted().collect::<String>())
        .collect_vec();

    let unique_words = words_sorted.iter().unique().collect_vec();

    words_sorted.len() == unique_words.len()
}

pub fn run(input: &str) -> (String, String) {
    (part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_passphrase_valid_1() {
        assert!(is_passphrase_valid("aa bb cc dd ee"));
    }

    #[test]
    fn test_is_passphrase_valid_2() {
        assert!(!is_passphrase_valid("aa bb cc dd aa"));
    }

    #[test]
    fn test_part1() {
        let input = "aa bb cc dd ee\n\
                   aa bb cc dd aa\n\
                   aa bb cc dd aaa";

        assert_eq!(part1(input), "2");
    }

    #[test]
    fn test_is_passphrase_valid_2_1() {
        assert!(is_passphrase_valid_2("iiii oiii ooii oooi oooo"));
    }

    #[test]
    fn test_is_passphrase_valid_2_2() {
        assert!(!is_passphrase_valid_2("oiii ioii iioi iiio"));
    }

    #[test]
    fn test_part2() {
        let input = "abcde fghij\n\
                   abcde xyz ecdab\n\
                   a ab abc abd abf abj\n\
                   iiii oiii ooii oooi oooo\n\
                   oiii ioii iioi iiio";

        assert_eq!(part2(input), "3");
    }
}
