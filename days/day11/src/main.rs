use std::char;

#[derive(Debug, PartialEq, Eq)]
struct Password(String);

impl Password {
    fn increment_char(c: char) -> (char, bool) {
        if c == 'z' {
            ('a', true)
        } else {
            (char::from_u32(c as u32 + 1).unwrap_or('a'), false)
        }
    }

    fn increment(&self) -> Self {
        let password = self.0.clone();
        let (reversed_password, _carry) =
            password
                .chars()
                .rev()
                .fold((String::new(), true), |(mut s, carry), c| {
                    let (c, carry) = if carry {
                        Self::increment_char(c)
                    } else {
                        (c, false)
                    };
                    s.push(c);
                    (s, carry)
                });
        Password(reversed_password.chars().rev().collect::<String>())
    }

    fn contains_three_increasing_letters(&self) -> bool {
        self.0
            .chars()
            .collect::<Vec<char>>()
            .windows(3)
            .any(|c| (c[1] as u32) == (c[0] as u32 + 1) && (c[2] as u32) == (c[1] as u32 + 1))
    }

    fn contains_i_o_l(&self) -> bool {
        self.0.chars().any(|c| c == 'i' || c == 'o' || c == 'l')
    }

    fn contains_two_pairs(&self) -> bool {
        let mut count_pairs = 0;

        let mut chars = self.0.chars().peekable();
        while let Some(c) = chars.next() {
            if let Some(next) = chars.peek() {
                if c == *next {
                    count_pairs += 1;
                    chars.next();
                }
            }
        }
        count_pairs >= 2
    }

    fn is_valid(&self) -> bool {
        self.contains_three_increasing_letters()
            && !self.contains_i_o_l()
            && self.contains_two_pairs()
    }

    fn next_valid_password(self) -> Self {
        let password = self.increment();
        repeat_until(password, |pwd| pwd.is_valid(), |pwd| pwd.increment())
    }
}

fn repeat_until<T>(mut value: T, predicate: fn(&T) -> bool, transform: fn(T) -> T) -> T {
    while !predicate(&value) {
        value = transform(value);
    }
    value
}

fn main() {
    let input = Password("vzbxkghb".to_string());
    let new_password = input.next_valid_password();
    println!("Part 1 = {}", new_password.0);
    println!("Part 2 = {}", new_password.next_valid_password().0);
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    #[test]
    fn part1_test_increment() {
        let input = Password("xx".to_string());
        assert_eq!(input.increment().0, "xy".to_string());
        let input = Password("xy".to_string());
        assert_eq!(input.increment().0, "xz".to_string());
        let input = Password("xz".to_string());
        assert_eq!(input.increment().0, "ya".to_string());
        let input = Password("ya".to_string());
        assert_eq!(input.increment().0, "yb".to_string());
    }

    #[test]
    fn part1_first_not_second_requirement() {
        let input = Password("hijklmmn".to_string());
        assert!(input.contains_three_increasing_letters());
        assert!(input.contains_i_o_l());
    }

    #[test]
    fn part1_third_not_first_requirement() {
        let input = Password("abbceffg".to_string());
        assert!(input.contains_two_pairs());
        assert!(!input.contains_three_increasing_letters());
    }

    #[test]
    fn part1_not_third_requirement() {
        let input = Password("abbcegjk".to_string());
        assert!(!input.contains_two_pairs());
    }

    #[test]
    fn part1_next_password_1() {
        let input = Password("abcdefgh".to_string());
        assert_eq!(input.next_valid_password().0, "abcdffaa".to_string());
    }

    #[test]
    fn part1_next_password_2() {
        let input = Password("ghijklmn".to_string());
        assert_eq!(input.next_valid_password().0, "ghjaabcc".to_string());
    }
}
