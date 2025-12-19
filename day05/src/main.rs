use std::{env::args, fs::read_to_string, io, ops::RangeInclusive};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let input = read_to_string(filename)?;
    let (mut ranges, ids) = process_input(&input);
    let part_1_total = count_fresh(ids, &ranges);
    println!("part 1: {part_1_total}");

    let part_2_total = total_valid(&mut ranges);
    println!("part 2: {part_2_total}");

    Ok(())
}

type Ranges = Vec<RangeInclusive<u64>>;

fn sort_ranges(ranges: &mut Ranges) -> &mut Ranges {
    ranges.sort_by(|a, b| {
        if a.start() < b.start() {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    ranges
}

fn merge_ranges(ranges: &mut Ranges) -> Ranges {
    // 3-5
    // 10-14
    // 16-20
    // 12-18
    // -> 3-5,10-20
    sort_ranges(ranges);

    let mut merged: Ranges = vec![];
    // if no overlap, add to merged
    // if overlap, extend then check against more
    ranges.iter().enumerate().for_each(|(index, current)| {
        // if merged prev
        let mut current = current.clone();
        let mut should_check = true;

        for prev in &merged {
            // current range completely in prev
            if current.start() >= prev.start() && current.end() <= prev.end() {
                should_check = false;
            }
        }

        if should_check {
            ranges.iter().skip(index + 1).for_each(|next| {
                // if a.end > b.start && a.end < b.end
                if current.start() <= next.start()
                    && (current.end() >= next.start()
                    // eg 2-2 3-5 - should extend
                        || current.end() + 1 >= *next.start())
                {
                    // a.end is in b range
                    // b.start = lesser of a.start and b.start
                    current = *current.start().min(next.start())..=*next.end().max(current.end());
                }
            });

            merged.push(current);
        }
    });

    merged
}

fn count_fresh(ids: &str, ranges: &Ranges) -> u64 {
    let mut count = 0;
    for id in ids.lines() {
        let id = id.parse::<u64>().expect("Id not a number");
        if is_fresh(id, ranges) {
            count += 1;
        }
    }

    count
}

fn process_input(input: &str) -> (Ranges, &str) {
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

    (merge_ranges(&mut ranges), data)
}

fn is_fresh(id: u64, ranges: &Ranges) -> bool {
    for range in ranges {
        if range.contains(&id) {
            return true;
        }
    }
    false
}

fn total_valid(ranges: &mut Ranges) -> u64 {
    sort_ranges(ranges);
    let mut gaps = 0;

    ranges.windows(2).for_each(|pair| {
        // 3..5
        // 10..20
        // = 4
        // assumes no overlap
        let gap = (pair[1].start() - pair[0].end()) - 1;
        gaps += gap;
    });

    let total_distance =
        ranges.last().expect("No ranges").end() - (ranges.first().expect("No ranges").start() - 1);

    total_distance - gaps
}

#[cfg(test)]
mod test {
    use crate::{count_fresh, is_fresh, merge_ranges, process_input, total_valid};

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
                vec![3..=5, 10..=20],
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

    #[test]
    fn should_merge_ranges() {
        assert_eq!(merge_ranges(&mut vec![1..=10, 2..=9]), vec![1..=10]);
        assert_eq!(
            merge_ranges(&mut vec![3..=5, 10..=14, 16..=20, 12..=18]),
            vec![3..=5, 10..=20]
        );
        assert_eq!(
            merge_ranges(&mut vec![
                3..=5,
                10..=10,
                10..=14,
                14..=14,
                14..=20,
                12..=18
            ]),
            vec![3..=5, 10..=20]
        );
        assert_eq!(
            merge_ranges(&mut vec![3..=5, 10..=14, 11..=12, 14..=20, 12..=18]),
            vec![3..=5, 10..=20]
        );
        assert_eq!(
            merge_ranges(&mut vec![
                35269914317143..=39695782939342,
                41365168848672..=41365168848672,
                41365168848672..=49031435034747,
                555240684135725..=555240684135725,
                555240684135726..=560251920336867
            ]),
            vec![
                35269914317143..=39695782939342,
                41365168848672..=49031435034747,
                555240684135725..=560251920336867
            ]
        );
    }

    #[test]
    fn it_should_count_in_ranges() {
        let mut ranges = vec![3..=5, 10..=20];
        assert_eq!(total_valid(&mut ranges), 14);

        let mut ranges = vec![3..=5, 10..=20, 22..=23];
        assert_eq!(total_valid(&mut ranges), 16);
    }
}
