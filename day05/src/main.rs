use std::{
    env::args,
    fs::read_to_string,
    io,
    ops::{RangeFrom, RangeInclusive},
};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let input = read_to_string(filename)?;
    let (ranges, ids) = process_input(&input);
    let part_1_total = count_fresh(ids, &ranges);
    println!("part 1: {part_1_total}");

    Ok(())
}

fn count_fresh(ids: &str, ranges: &Vec<RangeInclusive<u64>>) -> u64 {
    let mut count = 0;
    for id in ids.lines() {
        let id = id.parse::<u64>().expect("Id not a number");
        if is_fresh(id, ranges) {
            count += 1;
        }
    }

    count
}

fn process_input(input: &str) -> (Vec<RangeInclusive<u64>>, &str) {
    let tmp: Vec<&str> = input.split("\n\n").collect();
    let range_strs = tmp[0];
    let data = tmp[1];

    let mut ranges = vec![];

    for range in range_strs.lines() {
        let limits: Vec<u64> = range
            .split("-")
            .map(|n| n.parse::<u64>().expect("Not a number"))
            .collect();
        ranges.push(limits[0]..=limits[1]);
    }

    (ranges, data)
}

fn is_fresh(id: u64, ranges: &Vec<RangeInclusive<u64>>) -> bool {
    for range in ranges {
        if range.contains(&id) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use crate::{count_fresh, is_fresh, process_input};

    #[test]
    fn it_should_process_input() {
        let example_input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(
            process_input(example_input),
            (
                vec![3..=5, 10..=14, 16..=20, 12..=18],
                "1
5
8
11
17
32"
            )
        );
    }

    #[test]
    fn it_should_check_freshness() {
        let ranges = vec![3..=5, 10..=14, 16..=20, 12..=18];

        assert!(!is_fresh(1, &ranges));
        assert!(!is_fresh(8, &ranges));
        assert!(!is_fresh(32, &ranges));
        assert!(is_fresh(5, &ranges));
        assert!(is_fresh(11, &ranges));
        assert!(is_fresh(17, &ranges));
    }

    #[test]
    fn it_should_count_fresh_ingredients() {
        let ranges = vec![3..=5, 10..=14, 16..=20, 12..=18];
        let ids = "1
5
8
11
17
32";

        assert_eq!(count_fresh(ids, &ranges), 3);
    }
}
