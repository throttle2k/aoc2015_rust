use common::read_input;

fn contains_three_vowels(input: &str) -> bool {
    input
        .chars()
        .filter(|c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        })
        .count()
        >= 3
}

fn contains_double_letter(input: &str) -> bool {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .any(|c| c[0] == c[1])
}

fn contains_disallowed_strings(input: &str) -> bool {
    input.contains("ab") || input.contains("cd") || input.contains("pq") || input.contains("xy")
}

fn is_nice(input: &str) -> bool {
    contains_three_vowels(input)
        && contains_double_letter(input)
        && !contains_disallowed_strings(input)
}

fn contains_pair_twice(input: &str) -> bool {
    (0..input.len() - 2).any(|i| {
        let pair = &input[i..i + 2];
        input[i + 2..].contains(pair)
    })
}

fn contains_repeated_letter_with_separator(input: &str) -> bool {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(3)
        .any(|c| c[0] == c[2])
}

fn is_nice_part2(input: &str) -> bool {
    contains_pair_twice(input) && contains_repeated_letter_with_separator(input)
}

fn main() {
    let input = read_input("day05.txt");
    let nice = input.lines().filter(|s| is_nice(s)).count();
    println!("Part 1 = {}", nice);
    let nice_part_2 = input.lines().filter(|s| is_nice_part2(s)).count();
    println!("Part 2 = {}", nice_part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_test_part1_ugknbfddgicrmopn() {
        let input = "ugknbfddgicrmopn";
        assert!(contains_three_vowels(input));
        assert!(contains_double_letter(input));
        assert!(!contains_disallowed_strings(input));
        assert!(is_nice(input));
    }

    #[test]
    fn day05_test_part1_aaa() {
        let input = "aaa";
        assert!(contains_three_vowels(input));
        assert!(contains_double_letter(input));
        assert!(!contains_disallowed_strings(input));
        assert!(is_nice(input));
    }

    #[test]
    fn day05_test_part1_jchzalrnumimnmhp() {
        let input = "jchzalrnumimnmhp";
        assert!(!contains_double_letter(input));
        assert!(!is_nice(input));
    }

    #[test]
    fn day05_test_part1_haegwjzuvuyypxyu() {
        let input = "haegwjzuvuyypxyu";
        assert!(contains_disallowed_strings(input));
        assert!(!is_nice(input));
    }

    #[test]
    fn day05_test_part1_dvszwmarrgswjxmb() {
        let input = "dvszwmarrgswjxmb";
        assert!(!contains_three_vowels(input));
        assert!(!is_nice(input));
    }

    #[test]
    fn day05_test_part2_qjhvhtzxzqqjkmpb() {
        let input = "qjhvhtzxzqqjkmpb";
        assert!(contains_pair_twice(input));
        assert!(contains_repeated_letter_with_separator(input));
        assert!(is_nice_part2(input));
    }

    #[test]
    fn day05_test_part2_xxyxx() {
        let input = "xxyxx";
        assert!(contains_pair_twice(input));
        assert!(contains_repeated_letter_with_separator(input));
        assert!(is_nice_part2(input));
    }

    #[test]
    fn day05_test_part2_uurcxstgmygtbstg() {
        let input = "uurcxstgmygtbstg";
        assert!(contains_pair_twice(input));
        assert!(!contains_repeated_letter_with_separator(input));
        assert!(!is_nice_part2(input));
    }

    #[test]
    fn day05_test_part2_ieodomkazucvgmuy() {
        let input = "ieodomkazucvgmuy";
        assert!(!contains_pair_twice(input));
        assert!(contains_repeated_letter_with_separator(input));
        assert!(!is_nice_part2(input));
    }
}
