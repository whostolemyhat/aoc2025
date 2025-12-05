use std::{env::args, fs::read_to_string, io};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let input = read_to_string(filename)?;

    let count = calc_password(50, &input);
    let click_count = calc_password_with_clicks(50, &input);

    println!("part 1: {count}");
    println!("part 2: {click_count}");

    Ok(())
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unimplemented!(),
        }
    }
}

fn calc_password(start: i32, input: &str) -> i32 {
    let mut pos = start;
    let mut count = 0;

    input.lines().for_each(|line| {
        let (direction, num) = line.split_at(1);

        let direction: Direction = direction.into();
        let num = num.parse::<i32>().expect("Failed to parse number");

        match direction {
            Direction::Left => {
                // subtract from pos
                pos -= num.rem_euclid(100);

                if pos < 0 {
                    pos += 100;
                }
            }
            Direction::Right => {
                // add to pos
                pos += num.rem_euclid(100);
                if pos > 99 {
                    pos -= 100;
                }
            }
        }

        if pos == 0 {
            count += 1;
        }
    });

    count
}

fn calc_password_with_clicks(start: i32, input: &str) -> i32 {
    let mut pos = start;
    let mut count = 0;

    input.lines().for_each(|line| {
        let (direction, num) = line.split_at(1);

        let direction: Direction = direction.into();
        let num = num.parse::<i32>().expect("Failed to parse number");

        match direction {
            Direction::Left => {
                // subtract from pos
                for _ in 1..=num {
                    pos -= 1;

                    if pos < 0 {
                        pos = 99;
                    }

                    if pos == 0 {
                        count += 1;
                    }
                }
            }
            Direction::Right => {
                // add to pos
                for _ in 1..=num {
                    pos += 1;

                    if pos > 99 {
                        pos = 0;
                    }

                    if pos == 0 {
                        count += 1;
                    }
                }
            }
        }
    });

    count
}

#[cfg(test)]
mod test {
    use crate::{calc_password, calc_password_with_clicks};

    const TEST_INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn it_should_count_zeroes() {
        let count = calc_password(50, TEST_INPUT);
        assert_eq!(count, 3);
    }

    #[test]
    fn it_should_handle_nums_larger_than_100() {
        let big_input = r#"L50
R200"#;
        let count = calc_password(50, big_input);
        assert_eq!(count, 2);
    }

    #[test]
    fn it_should_count_clicks() {
        let count = calc_password_with_clicks(50, TEST_INPUT);
        assert_eq!(count, 6);
    }

    #[test]
    fn it_should_count_clicks_with_large_nums() {
        let big_input = r#"L50
R200"#;
        let count = calc_password_with_clicks(50, big_input);
        assert_eq!(count, 3);
    }
}
