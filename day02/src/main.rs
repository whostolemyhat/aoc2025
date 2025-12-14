use itertools::Itertools;
use std::{env::args, fs::read_to_string, io};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let input = read_to_string(filename)?;

    let count = calc_ids(&input);
    let part_2_count = calc_ids_repeated(&input);

    println!("count {count}");
    println!("part 2 count {part_2_count}");

    Ok(())
}

fn calc_ids(input: &str) -> u64 {
    let ranges = process_input(input);
    let mut count = 0;

    for range in ranges {
        let range = process_range(range);

        for i in range.0..=range.1 {
            if check_num(&format!("{i}")) {
                count += i;
            }
        }
    }

    count
}

fn calc_ids_repeated(input: &str) -> u64 {
    let ranges = process_input(input);
    let mut count = 0;

    for range in ranges {
        let range = process_range(range);

        for i in range.0..=range.1 {
            if check_repeated(&format!("{i}")) {
                count += i;
            }
        }
    }

    count
}

fn process_input(input: &str) -> Vec<&str> {
    input.split(",").collect()
}

fn process_range(range: &str) -> (u64, u64) {
    range
        .split("-")
        .next_tuple()
        .map(|(a, b)| {
            (
                a.parse::<u64>().expect("not a number"),
                b.parse::<u64>().expect("Not a number"),
            )
        })
        .expect("Should be a pair of nums")
}

fn check_num(id: &str) -> bool {
    let len = id.chars().count();
    if !len.is_multiple_of(2) {
        return false;
    }

    let (first, last) = id.split_at(len / 2);
    first == last
}

fn check_repeated(id: &str) -> bool {
    // if the next instance of id substr is less than halfway, then there's a pattern
    let doubled = format!("{id}{id}");

    // start from index 1 using slice, then add 1 back to get correct index
    (doubled[1..].find(id).expect("No match in doubled") + 1) != id.len()
}

#[cfg(test)]
mod test {
    use crate::{calc_ids, calc_ids_repeated, check_num, check_repeated};

    #[test]
    fn it_should_match_longest_substr() {
        let result = check_num("1010");
        assert!(result);
        assert!(!check_num("1"));
        assert!(check_num("11"));
        assert!(!check_num("101"));
        assert!(check_num("38593859"));
        assert!(!check_num("12"));
        assert!(check_num("222222"));
    }

    #[test]
    fn it_should_do_the_example() {
        let example_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(calc_ids(example_input), 1227775554);
    }

    #[test]
    fn it_should_add_repeated_ids_part_2() {
        let example_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(calc_ids_repeated(example_input), 4174379265);
    }

    #[test]
    fn it_should_match_repeated() {
        assert!(check_repeated("11"));
        assert!(check_repeated("22"));
        assert!(check_repeated("999"));
        assert!(check_repeated("1188511885"));
        assert!(check_repeated("2121212121"));

        // invalid
        assert!(!check_repeated("12"));
        assert!(!check_repeated("121"));
        assert!(!check_repeated("1001"));
        assert!(!check_repeated("1021"));
    }
}
