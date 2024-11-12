use std::collections::HashSet;

use common::read_input;

struct Replacer(Vec<(String, String)>);

impl From<&str> for Replacer {
    fn from(input: &str) -> Self {
        let replacements = input
            .lines()
            .map(|line| line.split_once(" => ").unwrap())
            .fold(Vec::new(), |mut replacements, (from, to)| {
                replacements.push((from.to_string(), to.to_string()));
                replacements
            });
        Replacer(replacements)
    }
}

impl Replacer {
    fn replace(&self, input: &str) -> Vec<String> {
        fn replace_with(input: &str, from: &str, to: &str) -> Vec<String> {
            if let Some(idx) = input.find(from) {
                let mut result = vec![input.replacen(from, to, 1)];
                let prefix = input[0..idx + 1].to_string();
                replace_with(&input[idx + 1..], from, to)
                    .into_iter()
                    .for_each(|suffix| {
                        let mut new_string = prefix.clone();
                        new_string.push_str(&suffix);
                        result.push(new_string);
                    });
                result
            } else {
                vec![]
            }
        }

        self.0.iter().fold(vec![], |mut result, replacement| {
            result.append(&mut replace_with(input, &replacement.0, &replacement.1));
            let set: HashSet<String> = result.drain(..).collect();
            result.extend(set.into_iter());
            result
        })
    }

    fn reverse_replace(&self, target: &str) -> usize {
        let mut molecule = target.trim().to_string();
        let mut steps = 0;

        while molecule != "e" {
            let mut replaced = false;
            for (to, from) in &self.0 {
                if let Some(idx) = molecule.find(from) {
                    molecule = format!(
                        "{}{}{}",
                        &molecule[..idx],
                        to,
                        &molecule[idx + from.len()..]
                    );
                    steps += 1;
                    replaced = true;
                    break;
                }
            }
            if !replaced {
                panic!("No further reduction possible. Check the input for cycles.");
            }
        }
        steps
    }
}

fn main() {
    let input = read_input("day19.txt");
    let (replacements, molecule) = input.split_once("\n\n").unwrap();
    let replacer = Replacer::from(replacements);
    println!("Part 1 = {}", replacer.replace(molecule).len());
    println!("Part 2 = {}", replacer.reverse_replace(molecule));
}

#[cfg(test)]
mod day19_tests {
    use super::*;

    #[test]
    fn part1_hoh() {
        let input = r#"H => HO
H => OH
O => HH

HOH"#;
        let (replacements, molecule) = input.split_once("\n\n").unwrap();
        let replacer = Replacer::from(replacements);
        assert_eq!(replacer.replace(molecule).len(), 4);
    }

    #[test]
    fn part1_hohoho() {
        let input = r#"H => HO
H => OH
O => HH

HOHOHO"#;
        let (replacements, molecule) = input.split_once("\n\n").unwrap();
        let replacer = Replacer::from(replacements);
        assert_eq!(replacer.replace(molecule).len(), 7);
    }

    #[test]
    fn part2_hoh() {
        let input = r#"H => HO
H => OH
O => HH
e => H
e => O

HOH"#;
        let (replacements, molecule) = input.split_once("\n\n").unwrap();
        let replacer = Replacer::from(replacements);
        assert_eq!(replacer.reverse_replace(molecule), 3);
    }

    #[test]
    fn part2_hohoho() {
        let input = r#"H => HO
H => OH
O => HH
e => H
e => O

HOHOHO"#;
        let (replacements, molecule) = input.split_once("\n\n").unwrap();
        let replacer = Replacer::from(replacements);
        assert_eq!(replacer.reverse_replace(molecule), 6);
    }
}
