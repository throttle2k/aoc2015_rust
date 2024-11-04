use common::read_input;

struct Coord(usize, usize);

impl From<&str> for Coord {
    fn from(value: &str) -> Self {
        let mut splits = value.split(',');
        Coord(
            splits.next().unwrap().parse().unwrap(),
            splits.next().unwrap().parse().unwrap(),
        )
    }
}

enum Command {
    TurnOn,
    Toggle,
    TurnOff,
}

struct Instruction {
    command: Command,
    from: Coord,
    to: Coord,
}

struct Grid([bool; 1_000_000]);

impl Default for Grid {
    fn default() -> Self {
        let lights = [false; 1_000_000];
        Grid(lights)
    }
}

impl Grid {
    #[allow(dead_code)]
    fn get(&self, col: usize, row: usize) -> bool {
        self.0[row * 1000 + col]
    }

    fn apply(&mut self, i: Instruction) {
        for row in i.from.1..=i.to.1 {
            for col in i.from.0..=i.to.0 {
                let light = &mut self.0[row * 1000 + col];
                match i.command {
                    Command::TurnOn => *light = true,
                    Command::Toggle => *light = !*light,
                    Command::TurnOff => *light = false,
                }
            }
        }
    }

    fn count_lights(&self) -> usize {
        self.0.iter().filter(|&l| *l == true).count()
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let (command, rest) = if value.starts_with("turn on") {
            (Command::TurnOn, value.strip_prefix("turn on ").unwrap())
        } else if value.starts_with("toggle") {
            (Command::Toggle, value.strip_prefix("toggle ").unwrap())
        } else {
            (Command::TurnOff, value.strip_prefix("turn off ").unwrap())
        };
        let mut splits = rest.split(' ');
        let from = splits.next().unwrap().into();
        let _ = splits.next().unwrap();
        let to = splits.next().unwrap().into();
        Instruction { command, from, to }
    }
}

struct GridV2(Vec<u32>);

impl Default for GridV2 {
    fn default() -> Self {
        let lights = vec![0; 1_000_000];
        GridV2(lights)
    }
}

impl GridV2 {
    #[allow(dead_code)]
    fn get(&self, col: usize, row: usize) -> u32 {
        self.0[row * 1000 + col]
    }

    fn apply(&mut self, i: Instruction) {
        for row in i.from.1..=i.to.1 {
            for col in i.from.0..=i.to.0 {
                let light = &mut self.0[row * 1000 + col];
                match i.command {
                    Command::TurnOn => *light += 1,
                    Command::Toggle => *light += 2,
                    Command::TurnOff => *light = if *light == 0 { 0 } else { *light - 1 },
                }
            }
        }
    }

    fn total_brightness(&self) -> u32 {
        self.0.iter().sum()
    }
}

fn main() {
    let input = read_input("day06.txt");
    let mut grid = Grid::default();
    input.lines().for_each(|s| {
        let instruction = Instruction::from(s);
        grid.apply(instruction);
    });
    println!("Part 1 = {}", grid.count_lights());
    let mut grid = GridV2::default();
    input.lines().for_each(|s| {
        let instruction = Instruction::from(s);
        grid.apply(instruction);
    });
    println!("Part 1 = {}", grid.total_brightness());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06_test_turn_0_0() {
        let mut grid = Grid::default();
        let input = "turn on 0,0 through 0,0";
        let instruction = Instruction::from(input);
        assert_eq!(grid.count_lights(), 0);
        grid.apply(instruction);
        assert_eq!(grid.count_lights(), 1);
    }

    #[test]
    fn day06_test_turn_all_on() {
        let mut grid = Grid::default();
        let input = "turn on 0,0 through 999,999";
        let instruction = Instruction::from(input);
        assert_eq!(grid.count_lights(), 0);
        grid.apply(instruction);
        assert_eq!(grid.count_lights(), 1_000_000);
    }

    #[test]
    fn day06_test_toggle_first_row() {
        let mut grid = Grid::default();
        grid.apply(Instruction {
            command: Command::TurnOn,
            from: Coord(0, 0),
            to: Coord(499, 0),
        });
        assert_eq!(grid.count_lights(), 500);
        let input = "toggle 0,0 through 999,0";
        let instruction = Instruction::from(input);
        grid.apply(instruction);
        assert_eq!(grid.count_lights(), 500);
        assert_eq!(grid.get(0, 0), false);
        assert_eq!(grid.get(500, 0), true);
    }

    #[test]
    fn day06_test_turn_off_center() {
        let mut grid = Grid::default();
        grid.apply(Instruction {
            command: Command::TurnOn,
            from: Coord(0, 0),
            to: Coord(999, 999),
        });
        assert_eq!(grid.count_lights(), 1_000_000);
        let input = "turn off 499,499 through 500,500";
        let instruction = Instruction::from(input);
        grid.apply(instruction);
        assert_eq!(grid.count_lights(), 999_996);
        assert_eq!(grid.get(499, 499), false);
        assert_eq!(grid.get(499, 500), false);
        assert_eq!(grid.get(500, 499), false);
        assert_eq!(grid.get(500, 500), false);
        assert_eq!(grid.get(0, 0), true);
        assert_eq!(grid.get(0, 999), true);
        assert_eq!(grid.get(999, 0), true);
        assert_eq!(grid.get(999, 999), true);
    }

    #[test]
    fn day06_test_part_2_turn_on_0_0() {
        let input = "turn on 0,0 through 0,0";
        let mut grid = GridV2::default();
        let instruction = Instruction::from(input);
        assert_eq!(grid.total_brightness(), 0);
        grid.apply(instruction);
        assert_eq!(grid.total_brightness(), 1);
    }

    #[test]
    fn day06_test_part_2_toggle_all() {
        let input = "toggle 0,0 through 999,999";
        let mut grid = GridV2::default();
        let instruction = Instruction::from(input);
        assert_eq!(grid.total_brightness(), 0);
        grid.apply(instruction);
        assert_eq!(grid.total_brightness(), 2_000_000);
    }
}
