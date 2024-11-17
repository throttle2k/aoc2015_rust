use std::usize;

use common::read_input;

fn quantum_energy_of(presents: &[i64]) -> i64 {
    presents.iter().fold(1, |qe, p| qe * p)
}

fn combine(
    presents: Vec<i64>,
    group: Vec<i64>,
    mut min_l: usize,
    mut min_qe: i64,
    target_sum: i64,
) -> (usize, i64) {
    if target_sum == 0 {
        return (group.len(), quantum_energy_of(&group));
    }

    if group.len() > min_l || (presents.is_empty() && target_sum > 0) {
        return (usize::max_value(), i64::max_value());
    }

    (0..presents.len()).for_each(|idx| {
        let mut remaining = presents.to_vec();
        let present = remaining.remove(idx);
        let (current_l, current_qe) = if group.len() <= min_l && target_sum - present >= 0 {
            let mut group_with_present = group.clone();
            group_with_present.push(present);
            combine(
                remaining,
                group_with_present,
                min_l,
                min_qe,
                target_sum - present,
            )
        } else {
            (usize::max_value(), i64::max_value())
        };

        if current_l == min_l {
            min_qe = min_qe.min(current_qe);
        } else if current_l < min_l {
            min_l = current_l;
            min_qe = current_qe;
        };
    });
    (min_l, min_qe)
}

fn best_qe(presents: Vec<i64>, num_groups: i64) -> (usize, i64) {
    let target_sum = presents.iter().sum::<i64>() / num_groups;
    let mut presents = presents.clone();
    presents.sort_by(|a, b| b.cmp(&a));
    combine(
        presents,
        vec![],
        usize::max_value(),
        i64::max_value(),
        target_sum,
    )
}

fn main() {
    let input = read_input("day24.txt");
    let presents = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<i64>>();
    println!("Part 1 = {}", best_qe(presents.clone(), 3).1);
    println!("Part 2 = {}", best_qe(presents, 4).1);
}

#[cfg(test)]
mod day24_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"1
2
3
4
5
7
8
9
10
11"#;
        let presents = input
            .lines()
            .map(|line| line.parse().unwrap())
            .collect::<Vec<i64>>();
        assert_eq!(best_qe(presents, 3).1, 99);
    }

    #[test]
    fn part2() {
        let input = r#"1
2
3
4
5
7
8
9
10
11"#;
        let presents = input
            .lines()
            .map(|line| line.parse().unwrap())
            .collect::<Vec<i64>>();
        assert_eq!(best_qe(presents, 4).1, 44);
    }
}
