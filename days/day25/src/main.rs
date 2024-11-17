fn get_code_number(row: usize, col: usize) -> u64 {
    let number_after_rows = (1..=row).fold(1, |mut acc, row| {
        acc += row as u64 - 1;
        acc
    });
    let code_number = (2..=col).fold(number_after_rows, |mut acc, col| {
        acc += (col + row) as u64 - 1;
        acc
    });
    code_number
}

fn get_code(starting: u64, row: usize, col: usize) -> u64 {
    let number = get_code_number(row, col);
    if number == 1 {
        starting
    } else {
        (1..number).fold(starting, |mut acc, _n| {
            acc = (acc * 252533) % 33554393;
            acc
        })
    }
}

fn main() {
    println!("Part 1 = {}", get_code(20151125, 2981, 3075));
}

#[cfg(test)]
mod day25_tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        input = {(1, 1), (2, 1), (1, 2), (3, 1), (2, 2), (1, 3), (4, 1), (3, 2), (2, 3), (1, 4), (5, 1), (4, 2), (3, 3), (2, 4), (1, 5), (6, 1), (5, 2), (4, 3), (3, 4), (2, 5), (1, 6), (6, 2), (5, 3), (4, 4), (3, 5), (2, 6), (6, 3), (5, 4), (4, 5), (3, 6), (6, 4), (5, 5), (4, 6), (6, 5), (5, 6), (6, 6)},
        value = {20151125, 31916031, 18749137, 16080970, 21629792, 17289845, 24592653, 8057251, 16929656, 30943339, 77061, 32451966, 1601130, 7726640, 10071777, 33071741, 17552253, 21345942, 7981243, 15514188, 33511524, 6796745, 28094349, 9380097, 11661866, 4041754, 25397450, 6899651, 10600672, 16474243, 24659492, 9250759, 31527494, 1534922, 31663883, 27995004}
    )]
    fn part1(input: (usize, usize), value: u64) {
        let (row, col) = input;
        assert_eq!(get_code(20151125, row, col), value);
    }
}
