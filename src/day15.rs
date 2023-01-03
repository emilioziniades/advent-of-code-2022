use std::{collections::HashSet, fs};

const CHARACTERS: [char; 17] = [
    'S', 'e', 'n', 's', 'o', 'r', 'a', 't', 'c', 'l', 'b', 'i', 'x', 'y', '=', ':', ',',
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sensor {
    position: Point,
    beacon: Point,
}

pub fn count_non_beacons(file: &str, row: i32) -> usize {
    let lines = fs::read_to_string(file).unwrap();
    let sensors: Vec<Sensor> = lines
        .lines()
        .map(|line| {
            let numbers = line
                .replace(CHARACTERS, "")
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>();

            Sensor {
                position: Point {
                    x: numbers[0],
                    y: numbers[1],
                },
                beacon: Point {
                    x: numbers[2],
                    y: numbers[3],
                },
            }
        })
        .collect();

    let mut positions: HashSet<Point> = HashSet::new();

    for sensor in sensors {
        // does it reach row
        let beacon_distance = manhattan_distance(&sensor.position, &sensor.beacon);
        let row_distance = sensor.position.y.abs_diff(row);
        if beacon_distance >= row_distance {
            // count positions in relevant row
            let vert_distance = (beacon_distance - row_distance) as i32;
            let row_start = sensor.position.x - vert_distance;
            let row_end = sensor.position.x + vert_distance;
            positions.extend((row_start..=row_end).map(|x| Point { x, y: row }));

            if sensor.beacon.y == row {
                positions.remove(&sensor.beacon);
            }
        }
    }

    let mut xs = positions.iter().map(|pt| pt.x).collect::<Vec<i32>>();
    xs.sort();

    println!("{xs:#?}");
    println!("{}", xs.len());

    positions.len()
}

fn manhattan_distance(a: &Point, b: &Point) -> u32 {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

#[cfg(test)]
mod tests {
    use crate::{day15, fetch_input};

    #[test]
    fn count_non_beacons() {
        fetch_input(15);

        let tests = vec![
            ("example/day15.txt", 26, 10),
            ("input/day15.txt", 5_125_700, 2_000_000),
        ];

        for test in tests {
            let (file, want, row) = test;
            let got = day15::count_non_beacons(file, row);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }
}
