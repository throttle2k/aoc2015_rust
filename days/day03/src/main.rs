use core::panic;
use std::{collections::HashMap, ops::Add};

use common::read_input;

#[derive(Hash, Eq, PartialEq, Clone)]
struct House(i32, i32);

impl Add for House {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        House(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn visit(input: &str, houses: HashMap<House, usize>) -> HashMap<House, usize> {
    let (mut houses, _) = input
        .chars()
        .map(|c| match c {
            '^' => House(0, 1),
            '>' => House(1, 0),
            '<' => House(-1, 0),
            'v' => House(0, -1),
            _ => panic!("Unrecognized character {c}"),
        })
        .fold((houses, House(0, 0)), |(mut houses, current), step| {
            let next_house = current + step;
            *houses.entry(next_house.clone()).or_insert(1) += 1;
            (houses, next_house)
        });
    *houses.entry(House(0, 0)).or_insert(1) += 1;
    houses
}

fn robot_visit(input: &str) -> HashMap<House, usize> {
    let santa_map: String = input.chars().step_by(2).collect();
    let robo_santa_map: String = input.chars().skip(1).step_by(2).collect();
    let santa_visit = visit(santa_map.as_str(), HashMap::new());
    let robot_visit = visit(robo_santa_map.as_str(), santa_visit);
    robot_visit
}

fn main() {
    let input = read_input("day03.txt");
    let visit = visit(input.as_str(), HashMap::new());
    println!("Part 1 = {}", visit.keys().count());
    let robo_visit = robot_visit(input.as_str());
    println!("Part 2 = {}", robo_visit.keys().count());
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(
        input = { ">", "^>v<", "^v^v^v^v^v" },
        expected = { 2, 4, 2 }
    )]
    fn day03_test_part_1(input: &str, expected: usize) {
        let visit = visit(input, HashMap::new());
        assert_eq!(visit.keys().count(), expected);
    }

    #[parameterized(
        input = { "^v", "^>v<", "^v^v^v^v^v" },
        expected = { 3, 3, 11 }
    )]
    fn day03_test_part_2(input: &str, expected: usize) {
        let visit = robot_visit(input);
        assert_eq!(visit.keys().count(), expected);
    }
}
