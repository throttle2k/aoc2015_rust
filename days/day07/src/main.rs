use core::panic;
use std::{collections::HashMap, str::FromStr};

use common::read_input;

#[derive(Debug)]
enum Operation {
    And,
    Or,
    LShift(u8),
    Rshift(u8),
    Not,
}

#[derive(Debug)]
enum GateInput {
    Wire(String),
    Value(u16),
}

impl FromStr for GateInput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = s.parse::<u16>() {
            Ok(GateInput::Value(v))
        } else {
            Ok(GateInput::Wire(s.to_string()))
        }
    }
}

#[derive(Debug)]
struct Gate {
    input_1: GateInput,
    input_2: Option<GateInput>,
    operation: Operation,
}

fn parse_binary_gate(value: Vec<&str>) -> Gate {
    let (input_1, input_2, operation) = match value[1] {
        "AND" => (
            value[0].parse().unwrap(),
            Some(value[2].parse().unwrap()),
            Operation::And,
        ),
        "OR" => (
            value[0].parse().unwrap(),
            Some(value[2].parse().unwrap()),
            Operation::Or,
        ),
        "LSHIFT" => (
            value[0].parse().unwrap(),
            None,
            Operation::LShift(value[2].parse().unwrap()),
        ),
        "RSHIFT" => (
            value[0].parse().unwrap(),
            None,
            Operation::Rshift(value[2].parse().unwrap()),
        ),
        _ => todo!(),
    };
    Gate {
        input_1,
        input_2,
        operation,
    }
}

fn parse_unary_gate(value: Vec<&str>) -> Gate {
    let (input_1, input_2, operation) = match value[0] {
        "NOT" => (value[1].parse().unwrap(), None, Operation::Not),
        _ => todo!(),
    };
    Gate {
        input_1,
        input_2,
        operation,
    }
}

impl From<Vec<&str>> for Gate {
    fn from(value: Vec<&str>) -> Self {
        match &value.len() {
            3 => parse_binary_gate(value),
            2 => parse_unary_gate(value),
            _ => panic!("Unrecognized pattern {:?}", value),
        }
    }
}

impl Gate {
    fn get_output_signal(&self, circuit: &Circuit, cache: &mut HashMap<String, u16>) -> u16 {
        let input_1 = match &self.input_1 {
            GateInput::Wire(w) => circuit.signal_of(&w, cache),
            GateInput::Value(v) => *v,
        };
        let input_2 = match &self.input_2 {
            Some(GateInput::Wire(w)) => circuit.signal_of(&w, cache),
            Some(GateInput::Value(v)) => *v,
            None => 0,
        };
        match self.operation {
            Operation::And => input_1 & input_2,
            Operation::Or => input_1 | input_2,
            Operation::LShift(i) => input_1 << i,
            Operation::Rshift(i) => input_1 >> i,
            Operation::Not => !input_1,
        }
    }
}

#[derive(Debug)]
enum Input {
    Gate(Gate),
    Wire(String),
    Value(u16),
}

#[derive(Debug)]
struct Wire {
    identifier: String,
    input: Input,
}

impl From<&str> for Wire {
    fn from(value: &str) -> Self {
        let mut splits = value.split("->");
        let lhs = splits.next().unwrap().trim();
        let rhs = splits.next().unwrap().trim();
        let lhs = lhs.split(' ').collect::<Vec<&str>>();
        let input = match lhs.len() {
            1 => {
                if let Ok(v) = lhs.get(0).unwrap().parse::<u16>() {
                    Input::Value(v)
                } else {
                    Input::Wire(lhs.get(0).unwrap().to_string())
                }
            }
            2 | 3 => Input::Gate(lhs.into()),
            _ => panic!("Unrecognized pattern for LHS {:?}", lhs),
        };
        Wire {
            identifier: rhs.to_string(),
            input,
        }
    }
}

#[derive(Debug)]
struct Circuit {
    wires: Vec<Wire>,
}

impl From<&str> for Circuit {
    fn from(input: &str) -> Self {
        let wires: Vec<Wire> = input.lines().map(|s| Wire::from(s)).collect();
        Self { wires }
    }
}

impl Circuit {
    fn signal_of(&self, wire_id: &str, cache: &mut HashMap<String, u16>) -> u16 {
        if let Some(&cached_signal) = cache.get(wire_id) {
            return cached_signal;
        }
        let wire = self
            .wires
            .iter()
            .find(|wire| wire.identifier == wire_id)
            .expect(format! {"Unable to find wire with id {wire_id}"}.as_str());
        let signal = match &wire.input {
            Input::Gate(g) => {
                // println!("The input of {wire_id} is a GATE {:?}", g);
                g.get_output_signal(self, cache)
            }
            Input::Wire(w) => {
                // println!("The input of {wire_id} is a Wire {w}");
                self.signal_of(&w, cache)
            }
            Input::Value(v) => {
                // println!("The input of {wire_id} is a Signal {v}");
                *v
            }
        };
        cache.insert(wire_id.to_string(), signal);
        signal
    }
}

fn main() {
    let input = read_input("day07.txt");
    let circuit = Circuit::from(input.as_str());
    let mut cache = HashMap::new();
    let signal_of_a = circuit.signal_of("a", &mut cache);
    println!("Part 1 = {}", signal_of_a);
    let mut cache = HashMap::new();
    cache.insert("b".to_string(), signal_of_a);
    println!("Part 2 = {}", circuit.signal_of("a", &mut cache));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day07_test_part1() {
        let input = r#"123 -> x
        456 -> y
        x AND y -> d
        x OR y -> e
        x LSHIFT 2 -> f
        y RSHIFT 2 -> g
        NOT x -> h
        NOT y -> i"#;
        let circuit = Circuit::from(input);
        assert_eq!(circuit.signal_of("d", &mut HashMap::new()), 72);
        assert_eq!(circuit.signal_of("e", &mut HashMap::new()), 507);
        assert_eq!(circuit.signal_of("f", &mut HashMap::new()), 492);
        assert_eq!(circuit.signal_of("g", &mut HashMap::new()), 114);
        assert_eq!(circuit.signal_of("h", &mut HashMap::new()), 65412);
        assert_eq!(circuit.signal_of("i", &mut HashMap::new()), 65079);
        assert_eq!(circuit.signal_of("x", &mut HashMap::new()), 123);
        assert_eq!(circuit.signal_of("y", &mut HashMap::new()), 456);
    }
}
