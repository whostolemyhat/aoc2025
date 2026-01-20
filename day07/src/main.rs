use std::{collections::HashSet, env::args, fs::read_to_string, io};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let input = read_to_string(filename)?;
    let layout = Layout::from(&input);
    let part_1 = layout.calculate_splits();

    let part_2 = layout.calculate_part_2();

    println!("Part 1: {part_1}");
    println!("Part 1: {part_2}");

    Ok(())
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Debug)]
struct Layout<'a> {
    width: usize,
    height: usize,
    start: Position,
    splitters: HashSet<Position>,
    map: Vec<&'a str>,
}

impl<'a> Layout<'a> {
    fn get_neighbours(&self, pos: Position) -> Vec<Position> {
        let mut neighbours = vec![];

        if pos.y >= self.height {
            return vec![];
        }
        if self.splitters.contains(&Position {
            x: pos.x,
            y: pos.y + 1,
        }) {
            neighbours.push(Position {
                x: pos.x - 1,
                y: pos.y,
            });
            neighbours.push(Position {
                x: pos.x + 1,
                y: pos.y,
            });
        } else {
            neighbours.push(Position {
                x: pos.x,
                y: pos.y + 1,
            });
        }

        neighbours
    }

    fn from(input: &'a str) -> Self {
        let mut layout = Layout {
            width: 0,
            height: 0,
            start: Position { x: 0, y: 0 },
            splitters: HashSet::new(),
            map: vec![],
        };

        let mut count = 0;
        for line in input.lines() {
            layout.width = line.len();
            line.chars().enumerate().for_each(|(i, c)| {
                if c == 'S' {
                    layout.start = Position { x: i, y: count };
                } else if c == '^' {
                    layout.splitters.insert(Position { x: i, y: count });
                }
            });
            layout.map.push(line);
            count += 1;
        }
        layout.height = count;

        layout
    }

    fn calculate_splits(&self) -> u16 {
        let mut beams = HashSet::from([self.start]);
        let mut count = 0;
        for _ in 0..self.height {
            (count, beams) = self.step(beams, count);
        }

        count
    }

    fn step(&self, beams: HashSet<Position>, count: u16) -> (u16, HashSet<Position>) {
        let mut updated_beams = HashSet::new();
        let mut total = count;
        // get next line from beam
        // is (beam.x, beam.y + 1) a splitter?
        for beam in beams {
            if beam.y < self.height - 1 {
                if self.splitters.contains(&Position {
                    x: beam.x,
                    y: beam.y + 1,
                }) {
                    updated_beams.insert(Position {
                        x: beam.x - 1,
                        y: beam.y + 1,
                    });
                    updated_beams.insert(Position {
                        x: beam.x + 1,
                        y: beam.y + 1,
                    });

                    total += 1;
                } else {
                    updated_beams.insert(Position {
                        x: beam.x,
                        y: beam.y + 1,
                    });
                }
            }
        }
        // no = push new pos
        // yes = increment and add (beam.x - 1, beam.y) and (beam.x + 1, beam.y)
        // return beams
        (total, updated_beams)
    }

    fn _calculate_part_2_search(&self) -> usize {
        // breadth-first takes forever!
        // todo: memoise
        let mut queue = vec![];
        let mut count = 0;

        queue.push(self.start);

        while !queue.is_empty() {
            if let Some(current) = queue.pop() {
                // memo
                // reached end
                if current.y == self.height - 1 {
                    count += 1;
                } else {
                    let mut neighbours = self.get_neighbours(current);
                    queue.append(&mut neighbours);
                }
            }
        }

        count
    }

    /// use array to count no. times position taken
    /// go line by line through map so each position is only checked once
    fn calculate_part_2(&self) -> usize {
        let mut count = vec![0; self.width];

        // add start
        count[self.start.x] = 1;

        for line in self.map.iter() {
            for (col, c) in line.chars().enumerate() {
                if c == '^' {
                    // on splitter, update paths either side
                    // and reset
                    let current = count[col];
                    count[col - 1] += current;
                    count[col + 1] += current;
                    count[col] = 0;
                }
            }
        }

        count.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{Layout, Position};

    #[test]
    fn it_should_parse_to_layout() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        let expected = Layout {
            width: 15,
            height: 16,
            start: Position { x: 7, y: 0 },
            splitters: HashSet::from([
                Position { x: 6, y: 4 },
                Position { x: 10, y: 8 },
                Position { x: 3, y: 10 },
                Position { x: 8, y: 4 },
                Position { x: 9, y: 10 },
                Position { x: 7, y: 2 },
                Position { x: 12, y: 12 },
                Position { x: 1, y: 14 },
                Position { x: 9, y: 6 },
                Position { x: 5, y: 14 },
                Position { x: 9, y: 14 },
                Position { x: 7, y: 14 },
                Position { x: 13, y: 14 },
                Position { x: 2, y: 12 },
                Position { x: 3, y: 14 },
                Position { x: 7, y: 6 },
                Position { x: 11, y: 10 },
                Position { x: 6, y: 8 },
                Position { x: 5, y: 6 },
                Position { x: 4, y: 8 },
                Position { x: 6, y: 12 },
                Position { x: 5, y: 10 },
            ]),
            map: vec![],
        };
        assert_eq!(Layout::from(input), expected);
    }

    #[test]
    fn it_should_step() {
        let layout = Layout {
            width: 15,
            height: 16,
            start: Position { x: 7, y: 0 },
            splitters: HashSet::from([
                Position { x: 6, y: 4 },
                Position { x: 10, y: 8 },
                Position { x: 3, y: 10 },
                Position { x: 8, y: 4 },
                Position { x: 9, y: 10 },
                Position { x: 7, y: 2 },
                Position { x: 12, y: 12 },
                Position { x: 1, y: 14 },
                Position { x: 9, y: 6 },
                Position { x: 5, y: 14 },
                Position { x: 9, y: 14 },
                Position { x: 7, y: 14 },
                Position { x: 13, y: 14 },
                Position { x: 2, y: 12 },
                Position { x: 3, y: 14 },
                Position { x: 7, y: 6 },
                Position { x: 11, y: 10 },
                Position { x: 6, y: 8 },
                Position { x: 5, y: 6 },
                Position { x: 4, y: 8 },
                Position { x: 6, y: 12 },
                Position { x: 5, y: 10 },
            ]),
            map: vec![],
        };

        let beams = HashSet::from([Position { x: 7, y: 1 }]);
        assert_eq!(
            layout.step(beams, 0),
            (
                1,
                HashSet::from([Position { x: 6, y: 2 }, Position { x: 8, y: 2 }])
            )
        );

        assert_eq!(
            layout.step(
                HashSet::from([Position { x: 6, y: 2 }, Position { x: 8, y: 2 }]),
                1
            ),
            (
                1,
                HashSet::from([Position { x: 6, y: 3 }, Position { x: 8, y: 3 }])
            )
        )
    }

    #[test]
    fn it_should_calculate() {
        let layout = Layout {
            width: 15,
            height: 16,
            start: Position { x: 7, y: 0 },
            splitters: HashSet::from([
                Position { x: 6, y: 4 },
                Position { x: 10, y: 8 },
                Position { x: 3, y: 10 },
                Position { x: 8, y: 4 },
                Position { x: 9, y: 10 },
                Position { x: 7, y: 2 },
                Position { x: 12, y: 12 },
                Position { x: 1, y: 14 },
                Position { x: 9, y: 6 },
                Position { x: 5, y: 14 },
                Position { x: 9, y: 14 },
                Position { x: 7, y: 14 },
                Position { x: 13, y: 14 },
                Position { x: 2, y: 12 },
                Position { x: 3, y: 14 },
                Position { x: 7, y: 6 },
                Position { x: 11, y: 10 },
                Position { x: 6, y: 8 },
                Position { x: 5, y: 6 },
                Position { x: 4, y: 8 },
                Position { x: 6, y: 12 },
                Position { x: 5, y: 10 },
            ]),
            map: vec![],
        };
        assert_eq!(layout.calculate_splits(), 21);
    }

    #[test]
    fn it_should_check_all_paths_search() {
        let layout = Layout {
            width: 15,
            height: 16,
            start: Position { x: 7, y: 0 },
            splitters: HashSet::from([
                Position { x: 6, y: 4 },
                Position { x: 10, y: 8 },
                Position { x: 3, y: 10 },
                Position { x: 8, y: 4 },
                Position { x: 9, y: 10 },
                Position { x: 7, y: 2 },
                Position { x: 12, y: 12 },
                Position { x: 1, y: 14 },
                Position { x: 9, y: 6 },
                Position { x: 5, y: 14 },
                Position { x: 9, y: 14 },
                Position { x: 7, y: 14 },
                Position { x: 13, y: 14 },
                Position { x: 2, y: 12 },
                Position { x: 3, y: 14 },
                Position { x: 7, y: 6 },
                Position { x: 11, y: 10 },
                Position { x: 6, y: 8 },
                Position { x: 5, y: 6 },
                Position { x: 4, y: 8 },
                Position { x: 6, y: 12 },
                Position { x: 5, y: 10 },
            ]),
            map: vec![],
        };
        assert_eq!(layout._calculate_part_2_search(), 40);
    }
    #[test]
    fn it_should_check_all_paths() {
        let layout = Layout {
            width: 15,
            height: 16,
            start: Position { x: 7, y: 0 },
            splitters: HashSet::from([
                Position { x: 6, y: 4 },
                Position { x: 10, y: 8 },
                Position { x: 3, y: 10 },
                Position { x: 8, y: 4 },
                Position { x: 9, y: 10 },
                Position { x: 7, y: 2 },
                Position { x: 12, y: 12 },
                Position { x: 1, y: 14 },
                Position { x: 9, y: 6 },
                Position { x: 5, y: 14 },
                Position { x: 9, y: 14 },
                Position { x: 7, y: 14 },
                Position { x: 13, y: 14 },
                Position { x: 2, y: 12 },
                Position { x: 3, y: 14 },
                Position { x: 7, y: 6 },
                Position { x: 11, y: 10 },
                Position { x: 6, y: 8 },
                Position { x: 5, y: 6 },
                Position { x: 4, y: 8 },
                Position { x: 6, y: 12 },
                Position { x: 5, y: 10 },
            ]),
            map: vec![
                ".......S.......",
                "...............",
                ".......^.......",
                "...............",
                "......^.^......",
                "...............",
                ".....^.^.^.....",
                "...............",
                "....^.^...^....",
                "...............",
                "...^.^...^.^...",
                "...............",
                "..^...^.....^..",
                "...............",
                ".^.^.^.^.^...^.",
                "...............",
            ],
        };
        assert_eq!(layout.calculate_part_2(), 40);
    }
}
