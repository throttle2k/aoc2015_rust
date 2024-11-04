fn mine(key: &str, min_zeroes: usize) -> usize {
    let zeroes = "0".repeat(min_zeroes);
    (0..)
        .take_while(|i| {
            let code = format!("{key}{i}");
            let hash = md5::compute(code);
            !format!("{:x}", hash).starts_with(&zeroes)
        })
        .count()
}

fn main() {
    println!("Part 1 = {}", mine("iwrupvqb", 5));
    println!("Part 2 = {}", mine("iwrupvqb", 6));
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(
        input = { "abcdef", "pqrstuv" },
        expected = { 609043, 1048970 }
    )]
    fn day04_test_part_1(input: &str, expected: usize) {
        assert_eq!(mine(input, 5), expected);
    }
}
