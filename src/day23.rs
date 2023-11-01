use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs, iter, slice,
};

const N_ROUNDS: usize = 10;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn north(&self) -> Self {
        Self::new(self.x - 1, self.y)
    }

    fn north_east(&self) -> Self {
        Self::new(self.x - 1, self.y + 1)
    }

    fn east(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    fn south_east(&self) -> Self {
        Self::new(self.x + 1, self.y + 1)
    }

    fn south(&self) -> Self {
        Self::new(self.x + 1, self.y)
    }

    fn south_west(&self) -> Self {
        Self::new(self.x + 1, self.y - 1)
    }

    fn west(&self) -> Self {
        Self::new(self.x, self.y - 1)
    }

    fn north_west(&self) -> Self {
        Self::new(self.x - 1, self.y - 1)
    }

    fn neighbours(&self) -> [Self; 8] {
        [
            self.north(),
            self.north_east(),
            self.east(),
            self.south_east(),
            self.south(),
            self.south_west(),
            self.west(),
            self.north_west(),
        ]
    }
}

#[derive(Debug)]
struct Grove {
    elves: HashSet<Point>,
}

impl Grove {
    fn new(input: &str) -> Self {
        let elves: HashSet<Point> = input
            .lines()
            .enumerate()
            .flat_map(|(x, row)| {
                row.trim().chars().enumerate().filter_map(move |(y, cell)| {
                    if cell == '#' {
                        Some(Point::new(x.try_into().unwrap(), y.try_into().unwrap()))
                    } else {
                        None
                    }
                })
            })
            .collect();

        Self { elves }
    }

    fn try_move_elf(&self, elf_position: &Point, direction: &Direction) -> Option<Point> {
        match direction {
            Direction::North => {
                if self.is_occupied([
                    elf_position.north(),
                    elf_position.north_east(),
                    elf_position.north_west(),
                ]) {
                    None
                } else {
                    Some(elf_position.north())
                }
            }
            Direction::East => {
                if self.is_occupied([
                    elf_position.east(),
                    elf_position.north_east(),
                    elf_position.south_east(),
                ]) {
                    None
                } else {
                    Some(elf_position.east())
                }
            }
            Direction::South => {
                if self.is_occupied([
                    elf_position.south(),
                    elf_position.south_west(),
                    elf_position.south_east(),
                ]) {
                    None
                } else {
                    Some(elf_position.south())
                }
            }
            Direction::West => {
                if self.is_occupied([
                    elf_position.west(),
                    elf_position.south_west(),
                    elf_position.north_west(),
                ]) {
                    None
                } else {
                    Some(elf_position.west())
                }
            }
        }
    }

    fn is_occupied(&self, points: [Point; 3]) -> bool {
        points.iter().any(|point| self.elves.contains(point))
    }

    fn elf_has_neighbour(&self, elf_position: &Point) -> bool {
        elf_position
            .neighbours()
            .iter()
            .any(|neighbour| self.elves.contains(neighbour))
    }

    fn move_all_elves(&mut self, round: usize) -> usize {
        let mut proposed_movements: HashMap<Point, Vec<Point>> = HashMap::new();
        for elf in &self.elves {
            if !self.elf_has_neighbour(elf) {
                continue;
            }

            for direction in Direction::cycle().skip(round).take(4) {
                if let Some(new_point) = self.try_move_elf(elf, direction) {
                    proposed_movements.entry(new_point).or_default().push(*elf);
                    break;
                }
            }
        }

        let legal_movements = proposed_movements
            .iter()
            .filter(|(_dest, srcs)| srcs.len() == 1)
            .map(|(dest, srcs)| (*dest, srcs[0]));

        let mut n_movements = 0;

        for (next_point, current_point) in legal_movements {
            self.elves.remove(&current_point);
            self.elves.insert(next_point);
            n_movements += 1;
        }

        n_movements
    }
}

impl Display for Grove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.elves.iter().max_by_key(|elf| elf.x).unwrap().x;
        let max_y = self.elves.iter().max_by_key(|elf| elf.y).unwrap().y;

        for x in 0..=max_x {
            for y in 0..=max_y {
                if self.elves.contains(&Point::new(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn cycle() -> iter::Cycle<slice::Iter<'static, Direction>> {
        [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
        .iter()
        .cycle()
    }
}

pub fn count_empty_ground_tiles(filename: &str) -> isize {
    let input = fs::read_to_string(filename).unwrap();
    let mut grove = Grove::new(&input);

    for n in 0..N_ROUNDS {
        grove.move_all_elves(n);
    }

    let largest_x = grove.elves.iter().max_by_key(|elf| elf.x).unwrap().x;
    let smallest_x = grove.elves.iter().min_by_key(|elf| elf.x).unwrap().x;

    let largest_y = grove.elves.iter().max_by_key(|elf| elf.y).unwrap().y;
    let smallest_y = grove.elves.iter().min_by_key(|elf| elf.y).unwrap().y;

    let grove_area = (largest_x - smallest_x + 1) * (largest_y - smallest_y + 1);

    let occupied_cells: isize = grove.elves.len().try_into().unwrap();

    grove_area - occupied_cells
}

pub fn rounds_until_no_movement(filename: &str) -> usize {
    let input = fs::read_to_string(filename).unwrap();
    let mut grove = Grove::new(&input);

    for n in 0.. {
        let n_movements = grove.move_all_elves(n);

        if n_movements == 0 {
            return n + 1;
        }
    }

    unreachable!("elves will always move at least once");
}

#[cfg(test)]
mod tests {
    use crate::{day23, fetch_input};

    #[test]
    fn count_empty_ground_tiles() {
        fetch_input(23);
        let tests = vec![("example/day23.txt", 110), ("input/day23.txt", 3800)];

        for (filename, want) in tests {
            let got = day23::count_empty_ground_tiles(filename);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }

    #[test]
    fn rounds_until_no_movement() {
        fetch_input(23);
        let tests = vec![("example/day23.txt", 20), ("input/day23.txt", 916)];

        for (filename, want) in tests {
            let got = day23::rounds_until_no_movement(filename);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }
}
