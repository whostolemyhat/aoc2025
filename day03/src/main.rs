use itertools::Itertools;
use std::{env::args, fs::read_to_string, io};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let input = read_to_string(filename)?;

    let joltage = calc_joltage(&input);
    println!("{joltage}");

    let part_2_joltage = calc_joltage_part_2(&input);
    println!("part 2: {part_2_joltage}");

    Ok(())
}

fn calc_joltage(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        total += find_largest(line);
    }

    total
}
fn calc_joltage_part_2(input: &str) -> u64 {
    let mut total = 0;
    for line in input.lines() {
        total += find_twelve(line);
    }

    total
}

fn process_stack(stack: &mut IncreasingStack<u32>, input: &str) -> u64 {
    for line in input.lines() {
        let chars: Vec<u32> = line
            .chars()
            .map(|n| n.to_digit(10).expect("Not a number"))
            .collect();

        for num in chars {
            stack.push(num);
        }

        println!("{:?}", stack.data);
    }

    0
}

#[derive(Clone, Copy)]
struct Record {
    num: u32,
    index: usize,
}

fn find_largest(bank: &str) -> u32 {
    let mut first = Record { num: 0, index: 0 };
    let mut second = Record { num: 0, index: 0 };

    // first
    let chars: Vec<u32> = bank
        .chars()
        .map(|n| n.to_digit(10).expect("Not a number"))
        .collect();

    // don't check last - need to leave a space for second
    chars
        .iter()
        .enumerate()
        .take(chars.len() - 1)
        .for_each(|(i, num)| {
            if *num > first.num {
                first = Record {
                    num: *num,
                    index: i,
                };
            }
        });

    // second
    chars
        .iter()
        .enumerate()
        // results have to be in order
        .skip(first.index + 1)
        .for_each(|(i, num)| {
            if *num > second.num {
                second = Record {
                    num: *num,
                    index: i,
                };
            }
        });

    format!("{}{}", first.num, second.num)
        .parse::<u32>()
        .expect("Output failed")
}

fn find_twelve(bank: &str) -> u64 {
    // make stack
    let chars: Vec<u32> = bank
        .chars()
        .map(|n| n.to_digit(10).expect("Not a number"))
        .collect();

    let collection = vec![];
    let stack = find_x(&chars, collection, -1, 12);

    // std can only join on vec<str>
    Itertools::join(&mut stack.iter(), "")
        .parse::<u64>()
        .expect("Failed to parse result")
}

// create stack
// length of window
// iter over window
// add to stack
// get last = highest
// mark new start point
// if window < remaining array, just return

// recurse
fn find_x(chars: &Vec<u32>, mut collection: Vec<u32>, start: i32, to_find: usize) -> Vec<u32> {
    if to_find == 0 {
        return collection;
    }

    // create a window, to exclude the last found index and making sure there are
    // enough digits left over for the remainder (since digits have to stay in order)
    // we're finding 12 so make sure there are enough digits left over
    let end = chars.len() + 1 - to_find;

    let mut found = 0;
    let mut index = 0;
    let slice = &chars[(start + 1) as usize..end];

    slice.iter().enumerate().for_each(|(i, num)| {
        if *num > found {
            found = *num;
            index = i;
        }
    });

    collection.push(found);

    // index is for the slice, so add back chars index
    find_x(chars, collection, index as i32 + start + 1, to_find - 1)
}

struct IncreasingStack<T> {
    data: Vec<T>,
}

impl<T> IncreasingStack<T>
where
    T: PartialOrd,
{
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn push(&mut self, item: T) {
        match self.data.last() {
            Some(last) => {
                if item >= *last {
                    self.data.push(item);
                } else {
                    while self.data.last().is_some()
                        && *self.data.last().expect("failed to get last item") > item
                    {
                        self.data.pop();
                    }
                    self.data.push(item);
                }
            }
            None => {
                self.data.push(item);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{IncreasingStack, find_largest, find_twelve, process_stack};

    #[test]
    fn it_should_find_largest_2_digits() {
        assert_eq!(find_largest("987654321111111"), 98);
        assert_eq!(find_largest("811111111111119"), 89);
        assert_eq!(find_largest("234234234234278"), 78);
        assert_eq!(find_largest("818181911112111"), 92);
    }

    #[test]
    fn it_should_find_twelve_largest() {
        assert_eq!(find_twelve("987654321111111"), 987654321111);
        assert_eq!(find_twelve("811111111111119"), 811111111119);
        assert_eq!(find_twelve("234234234234278"), 434234234278);
        assert_eq!(find_twelve("818181911112111"), 888911112111);
    }

    #[test]
    fn stack_should_keep_numbers_in_order() {
        let mut stack = IncreasingStack::<i32>::new();
        stack.push(1);
        stack.push(9);
        stack.push(5);
        assert_eq!(stack.data, vec![1, 5]);

        let mut stack = IncreasingStack::<f32>::new();
        stack.push(9.0);
        stack.push(10.0);
        stack.push(9.1);
        stack.push(12.0);
        stack.push(15.0);

        assert_eq!(stack.data, vec![9.0, 9.1, 12.0, 15.0]);
    }

    #[test]
    fn stack_should_find_largest() {
        let mut stack = IncreasingStack::<u32>::new();
        assert_eq!(process_stack(&mut stack, "987654321111111"), 987654321111);
    }
}
