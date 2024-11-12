use std::collections::HashMap;

fn divisors_of(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();
    (1..=((n as f64).sqrt() as usize))
        .filter(|i| n % i == 0)
        .for_each(|i| {
            divisors.push(i);
            if i != n / i {
                divisors.push(n / i);
            }
        });
    divisors
}

fn presents_at_house(
    n: usize,
    memo: &mut HashMap<usize, u32>,
    multiplier: u32,
    limit: Option<usize>,
) -> u32 {
    if let Some(&presents) = memo.get(&n) {
        return presents;
    }
    let presents = divisors_of(n)
        .into_iter()
        .filter(|&d| limit.map_or(true, |limit| n / d <= limit))
        .map(|d| d as u32 * multiplier)
        .sum();
    memo.insert(n, presents);
    presents
}

fn find_house_with(presents: u32, multiplier: u32, limit: Option<usize>) -> usize {
    let mut n = 1;
    let mut memo = HashMap::new();
    while presents_at_house(n, &mut memo, multiplier, limit) < presents {
        n += 1;
    }
    n
}

fn main() {
    println!("Part 1 = {}", find_house_with(34000000, 10, None));
    println!("Part 2 = {}", find_house_with(34000000, 11, Some(50)));
}

#[cfg(test)]
mod day20_tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        house = {1, 2, 3, 4, 5, 6, 7, 8, 9},
        presents = {10, 30, 40, 70, 60, 120, 80, 150, 130}
    )]
    fn part1(house: usize, presents: u32) {
        assert_eq!(
            presents_at_house(house, &mut HashMap::new(), 10, None),
            presents
        );
    }
}
