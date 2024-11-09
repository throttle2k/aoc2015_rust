use core::panic;

use common::read_input;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Status {
    Flying(u32),
    Resting(u32),
}

#[derive(Debug)]
struct Runner {
    name: String,
    speed: u32,
    fly_time: u32,
    resting_time: u32,
    status: Status,
    distance: u32,
    points: u32,
}

impl From<&str> for Runner {
    fn from(input: &str) -> Self {
        let mut splits = input.split(' ');
        let name = splits.next().unwrap().to_string();
        splits.next();
        splits.next();
        let speed = splits.next().unwrap();
        let speed = speed.parse::<u32>().unwrap();
        splits.next();
        splits.next();
        let fly_time = splits.next().unwrap().parse::<u32>().unwrap();
        splits.next();
        splits.next();
        splits.next();
        splits.next();
        splits.next();
        splits.next();
        let resting_time = splits.next().unwrap().parse::<u32>().unwrap();
        Self {
            name,
            speed,
            fly_time,
            resting_time,
            status: Status::Flying(fly_time),
            distance: 0,
            points: 0,
        }
    }
}

#[derive(Debug)]
struct Race {
    runners: Vec<Runner>,
}

impl Race {
    fn init(input: String) -> Self {
        let runners = input
            .lines()
            .map(|line| Runner::from(line.trim()))
            .collect::<Vec<Runner>>();
        Self { runners }
    }

    fn assign_points(&mut self) {
        let fastest = self.fastest();
        let distance = fastest.distance;
        self.runners
            .iter_mut()
            .filter(|r| r.distance == distance)
            .for_each(|r| r.points += 1);
    }

    fn step(&mut self) {
        self.runners.iter_mut().for_each(|r| match r.status {
            Status::Flying(ft) if ft > 0 => {
                r.status = Status::Flying(ft - 1);
                r.distance += r.speed;
            }
            Status::Flying(0) => {
                r.status = Status::Resting(r.resting_time - 1);
            }
            Status::Resting(rt) if rt > 0 => {
                r.status = Status::Resting(rt - 1);
            }
            Status::Resting(0) => {
                r.status = Status::Flying(r.fly_time - 1);
                r.distance += r.speed;
            }
            _ => panic!("Unknown status"),
        });
        self.assign_points();
    }

    fn step_for(&mut self, seconds: u32) {
        (0..seconds).for_each(|_| self.step());
    }

    #[allow(dead_code)]
    fn status_of(&self, name: &str) -> (Status, u32, u32) {
        let reindeer = self.runners.iter().find(|r| r.name == name).unwrap();
        (reindeer.status.clone(), reindeer.distance, reindeer.points)
    }

    fn fastest(&self) -> &Runner {
        self.runners
            .iter()
            .max_by(|r1, r2| r1.distance.cmp(&r2.distance))
            .unwrap()
    }

    fn max_points(&self) -> &Runner {
        self.runners
            .iter()
            .max_by(|r1, r2| r1.points.cmp(&r2.points))
            .unwrap()
    }
}

fn main() {
    let input = read_input("day14.txt");
    let mut race = Race::init(input);
    race.step_for(2503);
    println!("{:?}", race);
    println!("Part 1 = {}", race.fastest().distance);
    println!("Part 2 = {}", race.max_points().points);
}

#[cfg(test)]
mod day14_tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        second = {1, 10, 11, 12, 138, 174, 1000},
        comet = {(Status::Flying(9), 14), (Status::Flying(0), 140), (Status::Resting(126), 140), (Status::Resting(125), 140), (Status::Flying(9), 154), (Status::Resting(100), 280), (Status::Resting(96), 1120)},
        dancer = {(Status::Flying(10), 16), (Status::Flying(1), 160), (Status::Flying(0), 176), (Status::Resting(161), 176), (Status::Resting(35), 176), (Status::Flying(10), 192), (Status::Resting(38), 1056)},
    )]
    fn part1_stepping(second: u32, comet: (Status, u32), dancer: (Status, u32)) {
        let input = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#;
        let mut race = Race::init(input.to_string());
        race.step_for(second);
        assert_eq!(
            (race.status_of("Comet").0, race.status_of("Comet").1),
            comet
        );
        assert_eq!(
            (race.status_of("Dancer").0, race.status_of("Dancer").1),
            dancer
        );
    }

    #[test]
    fn part1_fastest() {
        let input = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#;
        let mut race = Race::init(input.to_string());
        race.step_for(1000);
        assert_eq!(race.fastest().name, "Comet");
        assert_eq!(race.fastest().distance, 1120);
    }

    #[parameterized(
        second = {1, 140, 1000},
        comet_points = {0, 1, 312},
        dancer_points = {1, 139, 689},
    )]
    fn part2_stepping(second: u32, comet_points: u32, dancer_points: u32) {
        let input = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#;
        let mut race = Race::init(input.to_string());
        race.step_for(second);
        assert_eq!(race.status_of("Comet").2, comet_points);
        assert_eq!(race.status_of("Dancer").2, dancer_points);
    }
}
