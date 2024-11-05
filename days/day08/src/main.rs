use common::read_input;

fn escape_string(input: &str) -> String {
    let mut chars = input.chars();
    let mut escaped_string = String::new();
    loop {
        match chars.next() {
            Some('\\') => match chars.next() {
                Some('\\') => escaped_string.push('\\'),
                Some('"') => escaped_string.push('"'),
                Some('x') => {
                    let char_1 = chars.next().unwrap();
                    let char_2 = chars.next().unwrap();
                    let hex: String = [char_1, char_2].iter().collect();
                    let _ch = u8::from_str_radix(&hex, 16).map(|n| n as char).unwrap();
                    // Some characters are not counted (ex. umlaut) so i use a placeholder
                    escaped_string.push('_');
                }
                _ => unreachable!(),
            },
            Some(c) => escaped_string.push(c),
            None => return escaped_string,
        }
    }
}

fn encode_string(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '"' => "\\\"".to_string(),
            '\\' => "\\\\".to_string(),
            c => c.to_string(),
        })
        .collect()
}

fn get_lengths(input: &str) -> (usize, usize) {
    let escaped_string = escape_string(&input[1..input.len() - 1]);
    (input.len(), escaped_string.len())
}

fn get_lengths_part2(input: &str) -> (usize, usize) {
    let encoded_string = format! {"\"{}\"", encode_string(input)};
    (input.len(), encoded_string.len())
}

fn get_length_diff(input: &str) -> usize {
    let (code_length, string_length) = get_lengths(input);
    code_length - string_length
}

fn get_length_diff_part2(input: &str) -> usize {
    let (input_length, encoded_length) = get_lengths_part2(input);
    encoded_length - input_length
}

fn get_total_diff_length(input: &str) -> usize {
    input.lines().map(|line| get_length_diff(line)).sum()
}

fn get_total_diff_part2(input: &str) -> usize {
    input.lines().map(|line| get_length_diff_part2(line)).sum()
}

fn main() {
    let input = read_input("day08.txt");
    println!("Part 1 = {}", get_total_diff_length(input.as_str()));
    println!("Part 2 = {}", get_total_diff_part2(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day08_part1_test_empty_string() {
        let input = r#""""#;
        let (code_char, string_char) = get_lengths(input);
        assert_eq!(code_char, 2);
        assert_eq!(string_char, 0);
    }

    #[test]
    fn day08_part1_test_abc() {
        let input = r#""abc""#;
        let (code_char, string_char) = get_lengths(input);
        assert_eq!(code_char, 5);
        assert_eq!(string_char, 3);
    }

    #[test]
    fn day08_part1_test_escaped_character() {
        let input = r#""aaa\"aaa""#;
        let (code_char, string_char) = get_lengths(input);
        assert_eq!(code_char, 10);
        assert_eq!(string_char, 7);
    }

    #[test]
    fn day08_part1_test_escaped_characters() {
        let input = r#""\xa8br\x8bjr\"""#;
        let (code_char, string_char) = get_lengths(input);
        assert_eq!(code_char, 16);
        assert_eq!(string_char, 7);
    }

    #[test]
    fn day08_part1_test_escaped_ascii() {
        let input = r#""\x27""#;
        let (code_char, string_char) = get_lengths(input);
        assert_eq!(code_char, 6);
        assert_eq!(string_char, 1);
    }

    #[test]
    fn day08_part1_test_total_diff_length() {
        let input = r#"""
        "abc"
        "aaa\"aaa"
        "\x27""#;
        assert_eq!(get_total_diff_length(input), 12);
    }

    #[test]
    fn day08_part2_test_encode_empty() {
        let input = r#""""#;
        let encoded = format!("\"{}\"", encode_string(input));
        assert_eq!(encoded, r#""\"\"""#);
        let (string_length, encoded_length) = get_lengths_part2(input);
        assert_eq!(string_length, 2);
        assert_eq!(encoded_length, 6);
    }

    #[test]
    fn day08_part2_test_encode_abc() {
        let input = r#""abc""#;
        let encoded = format!("\"{}\"", encode_string(input));
        assert_eq!(encoded, r#""\"abc\"""#);
        let (string_length, encoded_length) = get_lengths_part2(input);
        assert_eq!(string_length, 5);
        assert_eq!(encoded_length, 9);
    }

    #[test]
    fn day08_part2_test_encode_escaped_slash() {
        let input = r#""aaa\"aaa""#;
        let encoded = format!("\"{}\"", encode_string(input));
        assert_eq!(encoded, r#""\"aaa\\\"aaa\"""#);
        let (string_length, encoded_length) = get_lengths_part2(input);
        assert_eq!(string_length, 10);
        assert_eq!(encoded_length, 16);
    }

    #[test]
    fn day08_part2_test_encode_escaped_ascii() {
        let input = r#""\x27""#;
        let encoded = format!("\"{}\"", encode_string(input));
        assert_eq!(encoded, r#""\"\\x27\"""#);
        let (string_length, encoded_length) = get_lengths_part2(input);
        assert_eq!(string_length, 6);
        assert_eq!(encoded_length, 11);
    }

    #[test]
    fn day08_part2_test_total_diff_part2() {
        let input = r#"""
        "abc"
        "aaa\"aaa"
        "\x27""#;
        assert_eq!(get_total_diff_part2(input), 19);
    }
}
