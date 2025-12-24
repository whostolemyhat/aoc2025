use std::{env::args, fs::read_to_string, io, str::FromStr};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let input = read_to_string(filename)?;

    let (ops, nums) = parse_input(&input);
    let result = do_homework(&ops, &nums);
    println!("part 1: {result}");

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

fn parse_input_part_2(input: &str) -> (Vec<Operation>, Vec<Vec<Vec<char>>>) {
    let mut ops: Vec<Operation> = vec![];
    let container: Vec<Vec<Vec<char>>> = input
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
        .map(|(_index, line)| line.split(" ").map(|el| el.chars().collect()).collect())
        .collect();

    println!("{container:?}");

    (ops, container)
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

#[cfg(test)]
mod test {
    use crate::{Operation, do_homework, parse_input, parse_input_part_2};

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
        // let expected_nums = vec![
        //     vec![4, 431, 623],
        //     vec![175, 581, 32],
        //     vec![8, 248, 369],
        //     vec![356, 24, 1],
        // ];

        let expected_nums = vec![
            vec![4, 431, 623],
            vec![175, 581, 32],
            vec![8, 248, 369],
            vec![356, 24, 1],
        ];
        // assert_eq!(parse_input_part_2(input), (expected_ops, expected_nums));
    }
}
