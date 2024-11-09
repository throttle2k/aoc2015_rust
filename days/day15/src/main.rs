use core::panic;
use std::collections::HashMap;

use common::read_input;

#[derive(Debug)]
struct Ingredient {
    #[allow(dead_code)]
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl From<&str> for Ingredient {
    fn from(input: &str) -> Self {
        let (name, properties) = input.split_once(':').unwrap();
        properties
            .trim()
            .split(',')
            .fold(Ingredient::new(name.to_string()), |mut ingredient, p| {
                let (property, value) = p.trim().split_once(' ').unwrap();
                let value = value.parse::<i32>().unwrap();
                match property {
                    "capacity" => ingredient.capacity = value,
                    "durability" => ingredient.durability = value,
                    "flavor" => ingredient.flavor = value,
                    "texture" => ingredient.texture = value,
                    "calories" => ingredient.calories = value,
                    s => panic!("Unknown property {s}"),
                };
                ingredient
            })
    }
}

impl Ingredient {
    fn new(name: String) -> Self {
        Ingredient {
            name,
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        }
    }
}

#[derive(Debug)]
struct Recipe {
    ingredients: Vec<Ingredient>,
}

impl From<String> for Recipe {
    fn from(input: String) -> Self {
        let ingredients = input
            .lines()
            .map(|line| Ingredient::from(line))
            .collect::<Vec<Ingredient>>();
        Self { ingredients }
    }
}

fn find_combinations(n: usize, target_sum: u32) -> Vec<Vec<u32>> {
    let mut result = Vec::new();
    let mut current = Vec::with_capacity(n);

    fn backtrack(
        result: &mut Vec<Vec<u32>>,
        current: &mut Vec<u32>,
        n: usize,
        target_sum: u32,
        start: u32,
    ) {
        if current.len() == n {
            if target_sum == 0 {
                result.push(current.clone());
            }
            return;
        }

        for i in start..=target_sum {
            current.push(i);
            backtrack(result, current, n, target_sum - i, 0);
            current.pop();
        }
    }

    backtrack(&mut result, &mut current, n, target_sum, 0);
    result
}

impl Recipe {
    fn max_score_for_100_teaspoons(&self, calories_count: Option<u32>) -> u32 {
        let combinations = find_combinations(self.ingredients.len(), 100);
        combinations
            .iter()
            .map(|list| {
                let properties = list.iter().enumerate().fold(
                    HashMap::<&str, i32>::new(),
                    |mut properties, (i, quantity)| {
                        let ingredient = self.ingredients.get(i).unwrap();
                        properties
                            .entry("capacity")
                            .and_modify(|v| *v += (*quantity as i32) * ingredient.capacity)
                            .or_insert((*quantity as i32) * ingredient.capacity);
                        properties
                            .entry("durability")
                            .and_modify(|v| *v += (*quantity as i32) * ingredient.durability)
                            .or_insert((*quantity as i32) * ingredient.durability);
                        properties
                            .entry("flavor")
                            .and_modify(|v| *v += (*quantity as i32) * ingredient.flavor)
                            .or_insert((*quantity as i32) * ingredient.flavor);
                        properties
                            .entry("texture")
                            .and_modify(|v| *v += (*quantity as i32) * ingredient.texture)
                            .or_insert((*quantity as i32) * ingredient.texture);
                        properties
                            .entry("calories")
                            .and_modify(|v| *v += (*quantity as i32) * ingredient.calories)
                            .or_insert((*quantity as i32) * ingredient.calories);
                        properties
                    },
                );
                if properties.values().any(|v| *v <= 0) {
                    0
                } else {
                    if let Some(value) = calories_count {
                        if *properties.get("calories").unwrap() != (value as i32) {
                            0
                        } else {
                            properties
                                .keys()
                                .filter(|k| **k != "calories")
                                .map(|k| properties.get(k).unwrap())
                                .fold(1, |product, v| product * v)
                        }
                    } else {
                        properties
                            .keys()
                            .filter(|k| **k != "calories")
                            .map(|k| properties.get(k).unwrap())
                            .fold(1, |product, v| product * v)
                    }
                }
            })
            .max_by(|a, b| a.cmp(&b))
            .unwrap()
            .try_into()
            .unwrap()
    }
}

fn main() {
    let input = read_input("day15.txt");
    let recipe = Recipe::from(input);
    println!("Part 1 = {}", recipe.max_score_for_100_teaspoons(None));
    println!("Part 2 = {}", recipe.max_score_for_100_teaspoons(Some(500)));
}

#[cfg(test)]
mod day15_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"#;
        let recipe = Recipe::from(input.to_string());
        assert_eq!(62842880, recipe.max_score_for_100_teaspoons(None));
    }

    #[test]
    fn part2() {
        let input = r#"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"#;
        let recipe = Recipe::from(input.to_string());
        assert_eq!(57600000, recipe.max_score_for_100_teaspoons(Some(500)));
    }
}
