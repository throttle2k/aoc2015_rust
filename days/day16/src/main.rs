use std::collections::HashMap;

use common::read_input;

#[derive(Debug)]
struct Sue {
    number: usize,
    properties: HashMap<String, u32>,
}

impl From<(&str, &str)> for Sue {
    fn from((sue, properties): (&str, &str)) -> Self {
        let (_name, number) = sue.split_once(' ').unwrap();
        let properties = properties.split(',').fold(HashMap::new(), |mut props, s| {
            let (property, quantity) = s.split_once(':').unwrap();
            props.insert(
                property.trim().to_string(),
                quantity.trim().parse::<u32>().unwrap(),
            );
            props
        });
        Self {
            number: number.parse::<usize>().unwrap(),
            properties,
        }
    }
}

impl Sue {
    fn match_props(&self, properties: &HashMap<String, u32>) -> bool {
        properties.keys().all(|k| {
            let prop = self.properties.get(k).unwrap_or(properties.get(k).unwrap());
            prop == properties.get(k).unwrap()
        })
    }

    fn match_props_v2(&self, properties: &HashMap<String, u32>) -> bool {
        properties.keys().all(|k| {
            let value = properties.get(k).unwrap();
            let more_than_value = value + 1;
            let less_than_value = if *value == 0 { 0 } else { value - 1 };
            let prop = self.properties.get(k).unwrap_or_else(|| match k.as_str() {
                "trees" | "cats" => &more_than_value,
                "pomeranians" | "goldfish" => &less_than_value,
                _ => value,
            });
            match k.as_str() {
                "trees" | "cats" => prop > value,
                "pomeranians" | "goldfish" => prop < value,
                _ => prop == value,
            }
        })
    }
}

#[derive(Debug)]
struct Aunts {
    sues: Vec<Sue>,
}

impl From<String> for Aunts {
    fn from(input: String) -> Self {
        let sues = input
            .lines()
            .map(|line| {
                let line = line.split_once(':').unwrap();
                Sue::from(line)
            })
            .collect::<Vec<Sue>>();
        Aunts { sues }
    }
}

impl Aunts {
    fn find_sue_gen(&self, properties: &HashMap<String, u32>) -> &Sue {
        self.sues
            .iter()
            .find(|sue| sue.match_props(properties))
            .unwrap()
    }

    fn find_sue(&self, properties: &HashMap<String, u32>) -> &Sue {
        self.sues
            .iter()
            .find(|sue| sue.match_props(properties))
            .unwrap()
    }

    fn find_sue_v2(&self, properties: &HashMap<String, u32>) -> &Sue {
        self.sues
            .iter()
            .find(|sue| sue.match_props_v2(properties))
            .unwrap()
    }
}

fn main() {
    let input = read_input("day16.txt");
    let aunts = Aunts::from(input);
    let mut properties = HashMap::new();
    properties.insert("children".to_string(), 3);
    properties.insert("cats".to_string(), 7);
    properties.insert("samoyeds".to_string(), 2);
    properties.insert("pomeranians".to_string(), 3);
    properties.insert("akitas".to_string(), 0);
    properties.insert("vizslas".to_string(), 0);
    properties.insert("goldfish".to_string(), 5);
    properties.insert("trees".to_string(), 3);
    properties.insert("cars".to_string(), 2);
    properties.insert("perfumes".to_string(), 1);
    println!("Part 1 = {}", aunts.find_sue(&properties).number);
    println!("Part 2 = {}", aunts.find_sue_v2(&properties).number);
}
