use common::read_input;

fn end_level(input: &str) -> i32 {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("Unrecognized character {c}"),
        })
        .sum()
}

fn position_of(floor: i32, input: &str) -> usize {
    let position = (1..=input.len())
        .map(|i| end_level(&input[0..i]))
        .take_while(|&f| f != floor)
        .count();
    position + 1
}

fn main() {
    let input = read_input("day01.txt");
    println!("Part 1 = {}", end_level(input.as_str()));
    println!("Part 2 = {}", position_of(-1, input.as_str()));
}

#[cfg(test)]
mod tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        input = { "(())", "()()", "(((", "(()(()(", "))(((((", "())", "))(", ")))", ")())())"},
        expected = { 0, 0, 3, 3, 3, -1, -1, -3, -3}
    )]
    fn day01_test_part_1(input: &str, expected: i32) {
        assert_eq!(end_level(input), expected);
    }

    #[parameterized(
        input = { ")", "()())" },
        expected = { 1, 5 }
    )]
    fn day01_test_part_2(input: &str, expected: usize) {
        assert_eq!(position_of(-1, input), expected);
    }
}
