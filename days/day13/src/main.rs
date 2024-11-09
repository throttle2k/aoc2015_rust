use core::panic;
use std::collections::HashMap;

use common::read_input;

fn permutations<T: Clone>(input: Vec<T>) -> Vec<Vec<T>> {
    if input.is_empty() {
        vec![vec![]]
    } else {
        let mut result = Vec::new();
        for i in 0..input.len() {
            let mut rest = input.clone();
            let el = rest.remove(i);
            let sub_permutations = permutations(rest);
            for mut perm in sub_permutations {
                perm.push(el.clone());
                result.push(perm);
            }
        }
        result
    }
}

#[derive(Debug)]
struct Table {
    happiness_table: HashMap<String, HashMap<String, i32>>,
}

fn parse_row(input: &str) -> (String, String, i32) {
    let mut splits = input.split(' ');
    let member_1 = splits.next().unwrap();
    splits.next();
    let sign = match splits.next().unwrap() {
        "lose" => -1,
        "gain" => 1,
        _ => panic!("Unrecognized sign"),
    };
    let value = splits.next().unwrap().parse::<i32>().unwrap() * sign;
    splits.next();
    splits.next();
    splits.next();
    splits.next();
    splits.next();
    splits.next();
    let member_2 = splits.next().unwrap().strip_suffix(".").unwrap();
    (member_1.to_string(), member_2.to_string(), value)
}

impl From<String> for Table {
    fn from(input: String) -> Self {
        let table = input.lines().map(|row| parse_row(row)).fold(
            HashMap::new(),
            |mut table, (member_1, member_2, value)| {
                table
                    .entry(member_1)
                    .and_modify(|neighbours: &mut HashMap<String, i32>| {
                        neighbours.entry(member_2.clone()).or_insert(value);
                    })
                    .or_insert({
                        let mut new_neighbour = HashMap::new();
                        new_neighbour.insert(member_2, value);
                        new_neighbour
                    });
                table
            },
        );
        Self {
            happiness_table: table,
        }
    }
}

impl Table {
    fn most_happiness(&self) -> (Vec<String>, i32) {
        let members = self
            .happiness_table
            .keys()
            .cloned()
            .collect::<Vec<String>>();
        let mut rounds = permutations(members);
        rounds
            .iter_mut()
            .for_each(|list| list.push(list.get(0).unwrap().clone()));
        if let Some((seats, value)) = rounds
            .iter()
            .map(|list| {
                let mut list = list.clone();
                let happiness_next: i32 = list
                    .windows(2)
                    .map(|seats| {
                        self.happiness_table
                            .get(&seats[0])
                            .unwrap()
                            .get(&seats[1])
                            .unwrap()
                    })
                    .sum();
                list.reverse();
                let happiness_prev: i32 = list
                    .windows(2)
                    .map(|seats| {
                        self.happiness_table
                            .get(&seats[0])
                            .unwrap()
                            .get(&seats[1])
                            .unwrap()
                    })
                    .sum();
                let happiness = happiness_next + happiness_prev;
                (list.clone(), happiness)
            })
            .max_by(|(_list1, happiness1), (_list2, happiness2)| happiness1.cmp(&happiness2))
        {
            (seats, value)
        } else {
            (Vec::new(), 0)
        }
    }

    fn add_member(&mut self, name: String, value: i32) {
        let members = self
            .happiness_table
            .keys()
            .cloned()
            .collect::<Vec<String>>();
        self.happiness_table.values_mut().for_each(|seat| {
            seat.insert(name.clone(), value);
        });
        let seat = members.iter().fold(HashMap::new(), |mut seats, member| {
            seats.insert(member.clone(), value);
            seats
        });
        self.happiness_table.insert(name, seat);
    }
}

fn main() {
    let input = read_input("day13.txt");
    let mut table = Table::from(input);
    let most_happiness = table.most_happiness();
    println!("Day 1 = {}", most_happiness.1);
    table.add_member("Me".to_string(), 0);
    let most_happiness = table.most_happiness();
    println!("Day 2 = {}", most_happiness.1);
}

#[cfg(test)]
mod day13_tests {
    use super::*;

    #[test]
    fn test_combine() {
        let input = vec![1, 2, 3, 4];
        let result = permutations(input);
        assert!(result.contains(&vec![1, 2, 3, 4]));
        assert!(result.contains(&vec![1, 2, 4, 3]));
        assert!(result.contains(&vec![1, 3, 2, 4]));
        assert!(result.contains(&vec![1, 3, 4, 2]));
        assert!(result.contains(&vec![1, 4, 2, 3]));
        assert!(result.contains(&vec![1, 4, 3, 2]));
        assert!(result.contains(&vec![2, 1, 3, 4]));
        assert!(result.contains(&vec![2, 1, 4, 3]));
        assert!(result.contains(&vec![2, 3, 1, 4]));
        assert!(result.contains(&vec![2, 3, 4, 1]));
        assert!(result.contains(&vec![2, 4, 1, 3]));
        assert!(result.contains(&vec![2, 4, 3, 1]));
        assert!(result.contains(&vec![3, 1, 2, 4]));
        assert!(result.contains(&vec![3, 1, 4, 2]));
        assert!(result.contains(&vec![3, 2, 1, 4]));
        assert!(result.contains(&vec![3, 2, 4, 1]));
        assert!(result.contains(&vec![3, 4, 1, 2]));
        assert!(result.contains(&vec![3, 4, 2, 1]));
        assert!(result.contains(&vec![4, 1, 2, 3]));
        assert!(result.contains(&vec![4, 1, 3, 2]));
        assert!(result.contains(&vec![4, 2, 1, 3]));
        assert!(result.contains(&vec![4, 2, 3, 1]));
        assert!(result.contains(&vec![4, 3, 1, 2]));
        assert!(result.contains(&vec![4, 3, 2, 1]));
    }

    #[test]
    fn part1_test() {
        let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#;
        let table = Table::from(input.to_string());
        let most_happiness = table.most_happiness();
        assert_eq!(most_happiness.1, 330);
    }
}
