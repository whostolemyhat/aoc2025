use itertools::Itertools;
use std::{env::args, fs::read_to_string, io, str::FromStr};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let input = read_to_string(filename)?;

    let (ops, nums) = parse_input(&input);
    let result = do_homework(&ops, &nums);
    println!("part 1: {result}");

    let (ops_part2, nums_part2) = parse_input_part_2(&input);

    let result_part_2 = do_homework_part_2(&ops_part2, &nums_part2);

    println!("part 2: {result_part_2}");

    Ok(())
}

#[derive(PartialEq, Eq, Debug)]
enum Operation {
    Multiply,
    Add,
}

impl FromStr for Operation {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::Multiply),
            "+" => Ok(Self::Add),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Not an op")),
        }
    }
}

fn parse_input(input: &str) -> (Vec<Operation>, Vec<Vec<u64>>) {
    let mut ops: Vec<Operation> = vec![];
    let container: Vec<Vec<u64>> = input
        .lines()
        .rev()
        .enumerate()
        .filter(|(index, line)| {
            // process operations and then filter out for num parsing
            if *index == 0 {
                line.split_whitespace().for_each(|el| {
                    let parsed = el.trim().parse::<Operation>().expect("Not an operation");
                    ops.push(parsed);
                });

                false
            } else {
                true
            }
        })
        .map(|(_index, line)| {
            line.split_whitespace()
                .map(|el| el.trim().parse::<u64>().expect("not a number"))
                .collect()
        })
        .collect();

    (ops, container)
}

fn parse_input_part_2(input: &str) -> (Vec<Operation>, Vec<Vec<u64>>) {
    let ops = input
        .lines()
        .last()
        .expect("no ops")
        .chars()
        .fold(vec![], |mut acc, el| {
            if el != ' ' {
                let parsed = String::from(el)
                    .parse::<Operation>()
                    .expect("Not an operation");
                acc.push(parsed);
            };

            acc
        });
    let container: Vec<Vec<char>> = input
        .lines()
        .rev()
        .skip(1)
        .map(|line| line.chars().collect())
        .collect();

    let flipped: Vec<Vec<char>> = transpose(container.into_iter().rev().collect());
    let mapped: Vec<String> = flipped
        .iter()
        .map(|arr| {
            arr.iter().fold(String::new(), |mut acc, c| {
                acc.push(*c);
                acc
            })
        })
        .collect();

    let mut grouped = vec![];

    for (_key, chunk) in &mapped.into_iter().chunk_by(|item| item.trim() != "") {
        grouped.push(chunk.collect::<Vec<String>>());
    }

    let parsed: Vec<Vec<u64>> = grouped
        .iter()
        .filter(|n| !(n.len() == 1 && n[0].trim() == ""))
        .map(|arr| {
            arr.iter()
                .map(|n| n.trim().parse::<u64>().expect("not a number"))
                .collect::<Vec<u64>>()
        })
        .collect();
    (ops, parsed)
}

fn do_homework_part_2(ops: &[Operation], nums: &[Vec<u64>]) -> u64 {
    nums.iter().enumerate().fold(0, |acc, (i, n)| {
        let op = &ops[i];
        let total = match op {
            Operation::Add => n.iter().sum(),
            Operation::Multiply => n
                .iter()
                .fold(0, |acc, num| if acc > 0 { acc * *num } else { *num }),
        };
        acc + total
    })
}

fn do_homework(ops: &[Operation], nums: &[Vec<u64>]) -> u64 {
    let mut results: Vec<u64> = nums[0].clone();

    for row in nums.iter().skip(1) {
        for (i, num) in row.iter().enumerate() {
            // get op
            let op = &ops[i];
            // get num
            // read current result
            let current = results[i];
            // apply op
            let total = match op {
                Operation::Add => current + num,
                Operation::Multiply => current * num,
            };
            results[i] = total;
        }
    }

    results.iter().sum()
}

// matrix transposition
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().expect("reached end of iter"))
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{Operation, do_homework, do_homework_part_2, parse_input, parse_input_part_2};

    #[test]
    fn it_should_parse_input() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let expected_ops = vec![
            Operation::Multiply,
            Operation::Add,
            Operation::Multiply,
            Operation::Add,
        ];
        // reversed
        let expected_nums = vec![
            vec![6, 98, 215, 314],
            vec![45, 64, 387, 23],
            vec![123, 328, 51, 64],
        ];
        assert_eq!(parse_input(input), (expected_ops, expected_nums));
    }

    #[test]
    fn it_should_do_some_maths() {
        let nums = vec![
            vec![6, 98, 215, 314],
            vec![45, 64, 387, 23],
            vec![123, 328, 51, 64],
        ];
        let ops = vec![
            Operation::Multiply,
            Operation::Add,
            Operation::Multiply,
            Operation::Add,
        ];

        let result = do_homework(&ops, &nums);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn it_should_parse_part_2() {
        let input = "23  58 29
56  73 87
822 82 75
337 66 17
*   + ";

        let expected_ops = vec![Operation::Multiply, Operation::Add];

        let expected_nums = vec![vec![2583, 3623, 27], vec![5786, 8326], vec![2871, 9757]];
        assert_eq!(parse_input_part_2(input), (expected_ops, expected_nums));
        // 279083313 + 14112 + 28012347
    }

    #[test]
    fn sum_homework_part_2() {
        let ops = vec![Operation::Multiply];
        let nums = vec![vec![1, 2, 3]];
        assert_eq!(do_homework_part_2(&ops, &nums), 6);

        let ops = vec![Operation::Multiply, Operation::Add];
        let nums = vec![vec![1, 2, 3], vec![4, 5]];
        assert_eq!(do_homework_part_2(&ops, &nums), 15);

        let ops = vec![Operation::Multiply, Operation::Add, Operation::Multiply];
        let nums = vec![vec![2583, 3623, 27], vec![5786, 8326], vec![2871, 9757]];

        assert_eq!(do_homework_part_2(&ops, &nums), 280698102);
    }
}
