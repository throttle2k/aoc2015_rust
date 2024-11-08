use std::collections::HashMap;

use common::read_input;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum JsonElement {
    Array(Vec<JsonElement>),
    Object(HashMap<String, JsonElement>),
    Value(i32),
    String(String),
}

fn string_to_matching(paren: char, input: &str) -> &str {
    let mut position = 0;
    input
        .chars()
        .try_fold(Vec::<char>::new(), |mut parens: Vec<char>, c| match c {
            '[' => {
                parens.push(c);
                position += 1;
                Some(parens)
            }
            '{' => {
                parens.push(c);
                position += 1;
                Some(parens)
            }
            ']' if c != paren => {
                let _ = parens.pop();
                position += 1;
                Some(parens)
            }
            '}' if c != paren => {
                let _ = parens.pop();
                position += 1;
                Some(parens)
            }
            c if c == paren && parens.len() > 1 => {
                let _ = parens.pop();
                position += 1;
                Some(parens)
            }
            c if c == paren && parens.len() == 1 => None,
            _ => {
                position += 1;
                Some(parens)
            }
        });
    &input[0..position + 1]
}

impl From<&str> for JsonElement {
    fn from(value: &str) -> Self {
        let value = value.trim();
        let number_re = Regex::new(r"^-?[0-9]+$").unwrap();
        let array_re = Regex::new(r"^\[.*\]$").unwrap();
        let object_re = Regex::new(r"^\{.*\}$").unwrap();
        if number_re.is_match(value) {
            JsonElement::Value(value.parse::<i32>().unwrap())
        } else if array_re.is_match(value) {
            let mut element_list = Vec::new();
            let mut inner = value.strip_prefix("[").unwrap().strip_suffix("]").unwrap();
            while inner.len() > 0 {
                let chunk = if inner.starts_with('[') {
                    string_to_matching(']', inner)
                } else if inner.starts_with('{') {
                    string_to_matching('}', inner)
                } else {
                    &inner.chars().take_while(|&c| c != ',').collect::<String>()
                };
                element_list.push(JsonElement::from(chunk));
                inner = inner.strip_prefix(chunk).unwrap();
                if !inner.is_empty() {
                    inner = inner.strip_prefix(',').unwrap();
                }
            }
            JsonElement::Array(element_list)
        } else if object_re.is_match(value) {
            let mut object = HashMap::new();
            let mut inner = value.strip_prefix("{").unwrap().strip_suffix("}").unwrap();
            while inner.len() > 0 {
                let (id, rest) = inner.split_once(':').unwrap();
                let val = if rest.starts_with('[') {
                    string_to_matching(']', rest)
                } else if rest.starts_with('{') {
                    string_to_matching('}', rest)
                } else {
                    &rest.chars().take_while(|&c| c != ',').collect::<String>()
                };
                object.insert(
                    id.strip_prefix('"')
                        .unwrap()
                        .strip_suffix('"')
                        .unwrap()
                        .to_string(),
                    JsonElement::from(val),
                );
                let mut prefix = id.to_string();
                prefix.push(':');
                prefix.push_str(val);
                inner = inner.strip_prefix(prefix.as_str()).unwrap();
                if !inner.is_empty() {
                    inner = inner.strip_prefix(',').unwrap();
                }
            }
            JsonElement::Object(object)
        } else {
            JsonElement::String(value.to_string())
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SantaJson(JsonElement);

impl From<&str> for SantaJson {
    fn from(value: &str) -> Self {
        SantaJson(value.into())
    }
}

fn jsum(element: &JsonElement, skip: Option<&str>) -> i32 {
    match element {
        JsonElement::Value(v) => *v,
        JsonElement::Array(vec) => vec.iter().map(|e| jsum(e, skip)).sum(),
        JsonElement::Object(o) => {
            if skip.is_some()
                && o.values().any(|v| match v {
                    JsonElement::String(s) => skip.unwrap() == s,
                    _ => false,
                })
            {
                0
            } else {
                o.values().map(|v| jsum(v, skip)).sum()
            }
        }
        JsonElement::String(_) => 0,
    }
}
fn santa_jsum(input: SantaJson, skip: Option<&str>) -> i32 {
    jsum(&input.0, skip)
}

fn main() {
    let input = read_input("day12.txt");
    let json = SantaJson::from(input.as_str());
    println!("Part 1 = {}", santa_jsum(json, None));
    let json = SantaJson::from(input.as_str());
    println!("Part 2 = {}", santa_jsum(json, Some("\"red\"")));
}

#[cfg(test)]
mod day12_tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(
        input_par = {']', '}', ']', '}', ']', ']'},
        input_string = {"[pippo]", "{pluto}", "[{topolino}]", "{[nonna_papera]}", "[[qui],[quo],[qua, tip]]", "[1,2,3],[4,5,6]"},
        expected = {"[pippo]", "{pluto}", "[{topolino}]", "{[nonna_papera]}", "[[qui],[quo],[qua, tip]]", "[1,2,3]"},
    )]
    fn matching_parenthesis(input_par: char, input_string: &str, expected: &str) {
        assert_eq!(string_to_matching(input_par, input_string), expected);
    }

    #[test]
    fn parse_value() {
        assert_eq!(SantaJson::from("-12"), SantaJson(JsonElement::Value(-12)));
    }

    #[test]
    fn parse_array() {
        assert_eq!(
            SantaJson::from("[3]"),
            SantaJson(JsonElement::Array(vec![JsonElement::Value(3)]))
        );
        assert_eq!(
            SantaJson::from("[[[3]]]"),
            SantaJson(JsonElement::Array(vec![JsonElement::Array(vec![
                JsonElement::Array(vec![JsonElement::Value(3)])
            ])]))
        );
        assert_eq!(
            SantaJson::from("[[1,2,3],[4,5,6]]"),
            SantaJson(JsonElement::Array(vec![
                JsonElement::Array(vec![
                    JsonElement::Value(1),
                    JsonElement::Value(2),
                    JsonElement::Value(3)
                ]),
                JsonElement::Array(vec![
                    JsonElement::Value(4),
                    JsonElement::Value(5),
                    JsonElement::Value(6)
                ])
            ]))
        )
    }

    #[test]
    fn parse_object() {
        let mut val = HashMap::new();
        val.insert("a".to_string(), JsonElement::Value(1));
        val.insert("b".to_string(), JsonElement::Value(2));
        assert_eq!(
            SantaJson::from("{\"a\":1,\"b\":2}"),
            SantaJson(JsonElement::Object(val))
        );
        let mut inner = HashMap::new();
        inner.insert("c".to_string(), JsonElement::Value(3));
        let mut outer = HashMap::new();
        outer.insert("a".to_string(), JsonElement::Value(1));
        outer.insert("b".to_string(), JsonElement::Object(inner));
        outer.insert("d".to_string(), JsonElement::Value(2));
        assert_eq!(
            SantaJson::from("{\"a\":1,\"b\":{\"c\":3},\"d\":2}"),
            SantaJson(JsonElement::Object(outer))
        )
    }

    #[parameterized(
        input = { "[1, 2, 3]", "{\"a\":2,\"b\":4}", "[[[3]]]", "{\"a\":{\"b\":4},\"c\":-1}", "{\"a\":[-1, 1]}", "[-1,{\"a\":1}]", "[]", "{}"},
        sum = { 6, 6, 3, 3, 0, 0, 0, 0}
    )]
    fn part1_test_sum(input: &str, sum: i32) {
        let json = SantaJson::from(input);
        assert_eq!(santa_jsum(json, None), sum);
    }

    #[parameterized(
        input = { "[1,2,3]", "[1,{\"c\":\"red\",\"b\":2},3]", "{\"d\":\"red\",\"e\":[1, 2, 3, 4],\"f\":5}", "[1,\"red\",5]"},
        sum = { 6, 4, 0, 6}
    )]
    fn part2_test_sum(input: &str, sum: i32) {
        let json = SantaJson::from(input);
        assert_eq!(santa_jsum(json, Some("\"red\"")), sum);
    }
}
