use common::read_input;

struct Present {
    length: usize,
    width: usize,
    height: usize,
}

impl Present {
    pub fn from_str(input: &str) -> Self {
        let mut dimensions = input.split('x');
        let length = dimensions.next().unwrap().parse::<usize>().unwrap();
        let width = dimensions.next().unwrap().parse::<usize>().unwrap();
        let height = dimensions.next().unwrap().parse::<usize>().unwrap();
        Present {
            length,
            width,
            height,
        }
    }

    fn bottom_perimeter(&self) -> usize {
        (self.length + self.width) * 2
    }

    fn side_perimeter(&self) -> usize {
        (self.width + self.height) * 2
    }

    fn front_perimeter(&self) -> usize {
        (self.length + self.height) * 2
    }

    fn bottom_area(&self) -> usize {
        self.length * self.width
    }

    fn side_area(&self) -> usize {
        self.width * self.height
    }

    fn front_area(&self) -> usize {
        self.length * self.height
    }

    fn volume(&self) -> usize {
        self.length * self.width * self.height
    }

    pub fn paper_needed(&self) -> usize {
        let surface = 2 * self.bottom_area() + 2 * self.side_area() + 2 * self.front_area();
        let slack = self
            .bottom_area()
            .min(self.side_area().min(self.front_area()));
        surface + slack
    }

    pub fn ribbon_needed(&self) -> usize {
        let ribbon = self
            .bottom_perimeter()
            .min(self.side_perimeter().min(self.front_perimeter()));
        let bow = self.volume();
        ribbon + bow
    }
}

fn main() {
    let input = read_input("day02.txt");
    let paper_needed: usize = input
        .lines()
        .map(|s| {
            let present = Present::from_str(s);
            present.paper_needed()
        })
        .sum();
    let ribbon_needed: usize = input
        .lines()
        .map(|s| {
            let present = Present::from_str(s);
            present.ribbon_needed()
        })
        .sum();
    println!("Part 1 = {}", paper_needed);
    println!("Part 2 = {}", ribbon_needed);
}

#[cfg(test)]
mod tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        input = { "2x3x4", "1x1x10" },
        expected = { 58, 43 }
    )]
    fn test_part_1(input: &str, expected: usize) {
        let present = Present::from_str(input);
        assert_eq!(present.paper_needed(), expected);
    }

    #[parameterized(
        input = { "2x3x4", "1x1x10" },
        expected = { 34, 14 }
    )]
    fn test_part_2(input: &str, expected: usize) {
        let present = Present::from_str(input);
        assert_eq!(present.ribbon_needed(), expected);
    }
}
