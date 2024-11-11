use std::isize;

use common::read_input;

struct Lights {
    grid: Vec<bool>,
    rows: usize,
    cols: usize,
    locked_corners: bool,
}

impl ToString for Lights {
    fn to_string(&self) -> String {
        let s = (0..self.rows)
            .map(|row| {
                let mut row = (0..self.cols)
                    .map(|col| {
                        let c = self.get_light(row, col);
                        match c {
                            true => '#',
                            false => '.',
                        }
                    })
                    .collect::<String>();
                row.push('\n');
                row
            })
            .collect::<String>();
        s.trim_end().to_string()
    }
}

impl From<&str> for Lights {
    fn from(input: &str) -> Self {
        let mut cols = 0;
        let mut rows = 0;
        let grid = input
            .lines()
            .flat_map(|row| {
                rows += 1;
                cols = 0;
                row.trim()
                    .chars()
                    .map(|c| {
                        cols += 1;
                        match c {
                            '.' => false,
                            '#' => true,
                            c => panic!("Unknown character {c}"),
                        }
                    })
                    .collect::<Vec<bool>>()
            })
            .collect::<Vec<bool>>();
        Self {
            grid,
            rows,
            cols,
            locked_corners: false,
        }
    }
}

impl Lights {
    fn with_locked_corners(mut self) -> Self {
        self.locked_corners = true;
        self
    }

    fn get_light(&self, row: usize, col: usize) -> &bool {
        if self.locked_corners && {
            (row == 0 && col == 0)
                || (row == 0 && col == self.cols - 1)
                || (row == self.rows - 1 && col == 0)
                || (row == self.rows - 1 && col == self.cols - 1)
        } {
            &true
        } else {
            self.grid.get(row * self.cols + col).unwrap()
        }
    }

    fn get_neighbors(&self, row: usize, col: usize) -> Vec<&bool> {
        let delta_rows: Vec<isize> = if row == 0 {
            vec![0, 1]
        } else if row == self.rows - 1 {
            vec![-1, 0]
        } else {
            vec![-1, 0, 1]
        };
        let delta_cols: Vec<isize> = if col == 0 {
            vec![0, 1]
        } else if col == self.cols - 1 {
            vec![-1, 0]
        } else {
            vec![-1, 0, 1]
        };
        let mut delta_neighbors: Vec<(isize, isize)> = Vec::new();
        delta_rows.iter().for_each(|&dr| {
            delta_cols.iter().for_each(|&dc| {
                if (dr, dc) != (0, 0) {
                    delta_neighbors.push((dr, dc));
                }
            })
        });
        let neighbors = delta_neighbors
            .iter()
            .map(|(dr, dc)| {
                let nr = (row as isize + dr) as usize;
                let nc = (col as isize + dc) as usize;
                self.get_light(nr, nc)
            })
            .collect::<Vec<_>>();
        neighbors
    }

    fn step(&mut self) {
        let next = (0..self.rows)
            .flat_map(|row| {
                (0..self.cols)
                    .map(|col| {
                        let light = self.get_light(row, col);
                        let neighbors = self.get_neighbors(row, col);
                        let count = neighbors.iter().filter(|&n| **n == true).count();
                        match light {
                            true => match count {
                                2 | 3 => true,
                                _ => false,
                            },
                            false => match count {
                                3 => true,
                                _ => false,
                            },
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        self.grid = next;
    }

    fn count_lights_on(&self) -> usize {
        (0..self.rows)
            .map(|row| {
                (0..self.cols)
                    .map(|col| match self.get_light(row, col) {
                        true => 1,
                        false => 0,
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

fn main() {
    let input = read_input("day18.txt");
    let mut lights = Lights::from(input.as_str());
    (0..100).for_each(|_| lights.step());
    println!("Part 1 = {}", lights.count_lights_on());
    let input = read_input("day18.txt");
    let mut lights = Lights::from(input.as_str()).with_locked_corners();
    (0..100).for_each(|_| lights.step());
    println!("Part 2 = {}", lights.count_lights_on());
}

#[cfg(test)]
mod day18_tests {
    use super::*;

    #[test]
    fn part1() {
        let initial_state = r#".#.#.#
...##.
#....#
..#...
#.#..#
####.."#;
        let mut lights = Lights::from(initial_state);
        assert_eq!(lights.to_string(), initial_state);

        let after_1_step = r#"..##..
..##.#
...##.
......
#.....
#.##.."#;
        lights.step();
        assert_eq!(lights.to_string(), after_1_step);

        let after_2_step = r#"..###.
......
..###.
......
.#....
.#...."#;
        lights.step();
        assert_eq!(lights.to_string(), after_2_step);

        let after_3_step = r#"...#..
......
...#..
..##..
......
......"#;
        lights.step();
        assert_eq!(lights.to_string(), after_3_step);

        let after_4_step = r#"......
......
..##..
..##..
......
......"#;
        lights.step();
        assert_eq!(lights.to_string(), after_4_step);
        assert_eq!(lights.count_lights_on(), 4);
    }

    #[test]
    fn part2() {
        let initial_state = r#"##.#.#
...##.
#....#
..#...
#.#..#
####.#"#;
        let mut lights = Lights::from(initial_state).with_locked_corners();
        assert_eq!(lights.to_string(), initial_state);

        let after_1_step = r#"#.##.#
####.#
...##.
......
#...#.
#.####"#;
        lights.step();
        assert_eq!(lights.to_string(), after_1_step);

        let after_2_step = r#"#..#.#
#....#
.#.##.
...##.
.#..##
##.###"#;
        lights.step();
        assert_eq!(lights.to_string(), after_2_step);

        let after_3_step = r#"#...##
####.#
..##.#
......
##....
####.#"#;
        lights.step();
        assert_eq!(lights.to_string(), after_3_step);

        let after_4_step = r#"#.####
#....#
...#..
.##...
#.....
#.#..#"#;
        lights.step();
        assert_eq!(lights.to_string(), after_4_step);

        let after_5_step = r#"##.###
.##..#
.##...
.##...
#.#...
##...#"#;
        lights.step();
        assert_eq!(lights.to_string(), after_5_step);
        assert_eq!(lights.count_lights_on(), 17);
    }
}
