use common::read_input;

fn find_combinations(containers: &[i32], target_volume: i32) -> Vec<Vec<i32>> {
    if target_volume == 0 {
        return vec![Vec::new()];
    }
    if target_volume < 0 || containers.is_empty() {
        return Vec::new();
    }
    let (first_container, other_containers) = containers.split_first().unwrap();
    let combinations_with_first =
        find_combinations(other_containers, target_volume - first_container);
    let combinations_with_first = combinations_with_first
        .iter()
        .map(|combination| {
            let mut updated = combination.clone();
            updated.insert(0, first_container.clone());
            updated
        })
        .collect::<Vec<_>>();

    let mut combinations_without_first = find_combinations(other_containers, target_volume).clone();
    let mut result = combinations_with_first.clone();
    result.append(&mut combinations_without_first);
    result
}

fn count_combinations(containers: &[i32], liters: i32) -> usize {
    find_combinations(&containers, liters).len()
}

fn count_min_combinations(containers: &[i32], liters: i32) -> usize {
    let combinations = find_combinations(&containers, liters);
    let min_count = combinations.iter().map(|c| c.len()).min().unwrap();
    combinations
        .iter()
        .filter(|c| c.len() == min_count)
        .collect::<Vec<_>>()
        .len()
}

fn main() {
    let input = read_input("day17.txt");
    let containers = input
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    println!("Part 1 = {}", count_combinations(&containers, 150));
    println!("Part 2 = {}", count_min_combinations(&containers, 150));
}

#[cfg(test)]
mod day17_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"20
        15
        10
        5
        5"#;
        let containers = input
            .lines()
            .map(|line| line.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(count_combinations(&containers, 25), 4);
    }

    #[test]
    fn part2() {
        let input = r#"20
        15
        10
        5
        5"#;
        let containers = input
            .lines()
            .map(|line| line.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(count_min_combinations(&containers, 25), 3);
    }
}
