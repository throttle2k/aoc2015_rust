fn look_and_say(look: String, times: usize) -> String {
    if times == 0 {
        return look;
    }
    let mut chars = look.chars().peekable();
    let mut say = String::new();

    while let Some(c) = chars.next() {
        let mut count = 1;

        while let Some(&next) = chars.peek() {
            if next == c {
                chars.next();
                count += 1;
            } else {
                break;
            }
        }
        say.push_str(&count.to_string());
        say.push(c);
    }
    look_and_say(say, times - 1)
}

fn main() {
    println!(
        "Part 1: {}",
        look_and_say("3113322113".to_string(), 50).len()
    );
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(look_and_say("1".to_string(), 5), "312211");
    }
}
