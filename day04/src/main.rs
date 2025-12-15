use std::{collections::HashSet, env::args, fs::read_to_string, io};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let input = read_to_string(filename)?;

    let total = calc_reachable(&input);
    println!("part 1: {total}");

    let part_2_total = remove(&input);
    println!("part 2: {part_2_total}");

    Ok(())
}

fn calc_reachable(input: &str) -> u16 {
    // must be fewer than 4 rolls of paper nearby
    // parse as map
    let map = Map::parse(input);
    let mut total = 0;

    for roll in &map.rolls {
        if map.count_neighbours(roll) < 4 {
            total += 1;
        }
    }
    total
}

fn remove(input: &str) -> usize {
    let mut map = Map::parse(input);
    let mut to_remove = can_remove(&map);
    let mut count = 0;

    while !to_remove.is_empty() {
        count += to_remove.len();
        for roll in to_remove {
            map.rolls.remove(&roll);
        }
        // recurse
        // to_remove = 0;
        to_remove = can_remove(&map);
    }

    count
}

fn can_remove(map: &Map) -> HashSet<Position> {
    let mut to_remove = HashSet::new();

    for roll in &map.rolls {
        if map.count_neighbours(roll) < 4 {
            to_remove.insert(roll.clone());
        }
    }

    to_remove
}

#[derive(PartialEq, Debug, Clone)]
struct Map {
    map: Vec<char>,
    width: usize,
    height: usize,
    rolls: HashSet<Position>,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Position(usize, usize);

impl Map {
    fn parse(input: &str) -> Self {
        let mut rolls = HashSet::new();
        let mut height = 0;

        let map: Vec<char> = input
            .lines()
            .flat_map(|line| {
                let row = line.trim().chars().collect::<Vec<_>>();
                row.iter().enumerate().for_each(|(index, item)| {
                    if *item == '@' {
                        rolls.insert(Position(index, height));
                    }
                });
                height += 1;
                row
            })
            .collect();

        let width = map.len() / height;

        Map {
            width,
            height,
            map,
            rolls,
        }
    }

    fn count_neighbours(&self, pos: &Position) -> u8 {
        let mut count = 0;

        if pos.0 > 0 {
            // east
            if self.rolls.contains(&Position(pos.0 - 1, pos.1)) {
                count += 1;
            }

            if pos.1 > 0 {
                // northeast
                if self.rolls.contains(&Position(pos.0 - 1, pos.1 - 1)) {
                    count += 1;
                }
            }
            if pos.1 < self.height {
                // southeast
                if self.rolls.contains(&Position(pos.0 - 1, pos.1 + 1)) {
                    count += 1;
                }
            }
        }
        if pos.0 < self.width {
            // west
            if self.rolls.contains(&Position(pos.0 + 1, pos.1)) {
                count += 1;
            }
            if pos.1 > 0 {
                // northwest
                if self.rolls.contains(&Position(pos.0 + 1, pos.1 - 1)) {
                    count += 1;
                }
            }
            if pos.1 < self.height {
                // southwest
                if self.rolls.contains(&Position(pos.0 + 1, pos.1 + 1)) {
                    count += 1;
                }
            }
        }
        if pos.1 > 0 {
            // north
            if self.rolls.contains(&Position(pos.0, pos.1 - 1)) {
                count += 1;
            }
        }
        if pos.1 < self.height {
            // south
            if self.rolls.contains(&Position(pos.0, pos.1 + 1)) {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use crate::{calc_reachable, remove};

    #[test]
    fn it_should_find_reachable() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(calc_reachable(input), 13);
    }

    #[test]
    fn it_should_remove_rolls() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(remove(input), 43);
    }
}
