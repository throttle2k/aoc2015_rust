use std::{cmp::Ordering, collections::HashMap, hash::Hash};

use common::read_input;

#[derive(Debug, Eq, PartialEq)]
struct City {
    name: String,
    distances: HashMap<String, u32>,
}

impl Hash for City {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Hash::hash(&self.name, state)
    }
}

impl City {
    fn new(name: String) -> Self {
        City {
            name,
            distances: HashMap::new(),
        }
    }

    fn with_destination(mut self, city: String, distance: u32) -> Self {
        self.add_destination(city, distance);
        self
    }

    fn add_destination(&mut self, city: String, distance: u32) {
        self.distances
            .entry(city)
            .and_modify(|d| *d = distance)
            .or_insert(distance);
    }
}

#[derive(Debug)]
struct Roadmap {
    cities: Vec<City>,
}

impl Roadmap {
    fn traverse<F>(&self, from: &City, mut visited: Vec<String>, compare: F) -> u32
    where
        F: Fn((&String, &u32), (&String, &u32)) -> Ordering,
    {
        visited.push(from.name.clone());
        if self.cities.len() == visited.len() {
            0
        } else {
            let next = from
                .distances
                .iter()
                .filter(|(k, _v)| !visited.contains(k))
                .min_by(|&a, &b| compare(a, b))
                .map(|(k, _v)| k)
                .unwrap();
            from.distances.get(next).unwrap()
                + self.traverse(self.get_city(next.to_string()), visited, compare)
        }
    }

    fn get_city(&self, name: String) -> &City {
        self.cities.iter().find(|city| city.name == name).unwrap()
    }

    fn min_distance(&self) -> u32 {
        self.cities
            .iter()
            .map(|c| self.traverse(c, Vec::new(), |a, b| a.1.cmp(&b.1)))
            .min()
            .unwrap()
    }

    fn max_distance(&self) -> u32 {
        self.cities
            .iter()
            .map(|c| self.traverse(c, Vec::new(), |a, b| b.1.cmp(&a.1)))
            .max()
            .unwrap()
    }
}

impl From<&str> for Roadmap {
    fn from(value: &str) -> Self {
        let mut cities: HashMap<&str, City> = HashMap::new();
        value
            .lines()
            .map(|line| line.split(" = "))
            .for_each(|mut split| {
                let path = split.next().unwrap();
                let mut path = path.split(" to ");
                let (source, destination) = (path.next().unwrap(), path.next().unwrap());
                let distance = split.next().unwrap().parse::<u32>().unwrap();
                cities
                    .entry(source)
                    .and_modify(|city| city.add_destination(destination.to_string(), distance))
                    .or_insert(
                        City::new(source.to_string())
                            .with_destination(destination.to_string(), distance),
                    );
                cities
                    .entry(destination)
                    .and_modify(|city| city.add_destination(source.to_string(), distance))
                    .or_insert(
                        City::new(destination.to_string())
                            .with_destination(source.to_string(), distance),
                    );
            });
        let cities = cities.into_values().collect();

        Roadmap { cities }
    }
}

fn main() {
    let input = read_input("day09.txt");
    let map = Roadmap::from(input.as_str());
    println!("Part 1 = {}", map.min_distance());
    println!("Part 2 = {}", map.max_distance());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day09_part1_test() {
        let input = r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#;
        let map = Roadmap::from(input);

        assert_eq!(map.min_distance(), 605);
    }

    #[test]
    fn day09_part2_test() {
        let input = r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#;
        let map = Roadmap::from(input);

        assert_eq!(map.max_distance(), 982);
    }
}
