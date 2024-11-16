use common::read_input;

#[derive(Debug)]
struct Computer {
    reg_a: usize,
    reg_b: usize,
    instructions: Vec<Instruction>,
}

impl From<String> for Computer {
    fn from(value: String) -> Self {
        let instructions = value.lines().map(|line| Instruction::from(line)).collect();
        Self {
            reg_a: 0,
            reg_b: 0,
            instructions,
        }
    }
}

impl Computer {
    fn with_reg_a(mut self, reg_a: usize) -> Self {
        self.reg_a = reg_a;
        self
    }

    fn get_instruction(&self, index: isize) -> Option<&Instruction> {
        if index < 0 {
            None
        } else {
            self.instructions.get(index as usize)
        }
    }

    fn execute(&mut self) -> (usize, usize) {
        let mut index = 0;
        while let Some(instruction) = self.get_instruction(index) {
            dbg!(&self.reg_a, &self.reg_b);
            dbg!(instruction);
            index = match instruction {
                Instruction::Hlf(reg) => {
                    match reg.as_str() {
                        "a" => self.reg_a /= 2,
                        "b" => self.reg_b /= 2,
                        r => panic!("Unknown register {r}"),
                    }
                    index + 1
                }
                Instruction::Tpl(reg) => {
                    match reg.as_str() {
                        "a" => self.reg_a *= 3,
                        "b" => self.reg_b *= 3,
                        r => panic!("Unknown register {r}"),
                    }
                    index + 1
                }
                Instruction::Inc(reg) => {
                    match reg.as_str() {
                        "a" => self.reg_a += 1,
                        "b" => self.reg_b += 1,
                        r => panic!("Unknown register {r}"),
                    }
                    index + 1
                }
                Instruction::Jmp(offset) => index + offset,
                Instruction::Jie(reg, offset) => {
                    let must_jump = match reg.as_str() {
                        "a" => self.reg_a % 2 == 0,
                        "b" => self.reg_b % 2 == 0,
                        r => panic!("Unknown register {r}"),
                    };
                    if must_jump {
                        index + offset
                    } else {
                        index + 1
                    }
                }
                Instruction::Jio(reg, offset) => {
                    let must_jump = match reg.as_str() {
                        "a" => self.reg_a == 1,
                        "b" => self.reg_b == 1,
                        r => panic!("Unknown register {r}"),
                    };
                    if must_jump {
                        index + offset
                    } else {
                        index + 1
                    }
                }
            };
        }
        (self.reg_a, self.reg_b)
    }
}

#[derive(Debug)]
enum Instruction {
    Hlf(String),
    Tpl(String),
    Inc(String),
    Jmp(isize),
    Jie(String, isize),
    Jio(String, isize),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let (command, params) = value.split_once(' ').unwrap();
        match command {
            "hlf" => Instruction::Hlf(params.to_string()),
            "tpl" => Instruction::Tpl(params.to_string()),
            "inc" => Instruction::Inc(params.to_string()),
            "jmp" => Instruction::Jmp(params.parse().unwrap()),
            "jie" => {
                let (register, offset) = params.split_once(", ").unwrap();
                Instruction::Jie(register.to_string(), offset.parse().unwrap())
            }
            "jio" => {
                let (register, offset) = params.split_once(", ").unwrap();
                Instruction::Jio(register.to_string(), offset.parse().unwrap())
            }
            c => panic!("Unknown command {c}"),
        }
    }
}

fn main() {
    let input = read_input("day23.txt");
    let mut computer = Computer::from(input.clone());
    let (_reg_a, reg_b) = computer.execute();
    println!("Part 1 = {reg_b}");
    let mut computer = Computer::from(input).with_reg_a(1);
    let (_reg_a, reg_b) = computer.execute();
    println!("Part 2 = {reg_b}");
}

#[cfg(test)]
mod day23_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"inc a
jio a, +2
tpl a
inc a"#;
        let mut computer = Computer::from(input.to_string());
        let (reg_a, _reg_b) = computer.execute();
        assert_eq!(reg_a, 2);
    }
}
