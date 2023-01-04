use std::{collections::HashSet, fs, ops::RangeInclusive};

const CHARACTERS: [char; 17] = [
    'S', 'e', 'n', 's', 'o', 'r', 'a', 't', 'c', 'l', 'b', 'i', 'x', 'y', '=', ':', ',',
];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
    let input = fs::read_to_string(file).unwrap();
    let sensors: Vec<Sensor> = input
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

    let beacons_in_row = sensors
        .iter()
        .filter(|sensor| sensor.beacon.y == row)
        .map(|sensor| sensor.beacon)
        .collect::<HashSet<Point>>()
        .len();

    let mut ranges: Vec<RangeInclusive<i32>> = Vec::new();

    for sensor in sensors {
        // does it reach row
        let beacon_distance = manhattan_distance(&sensor.position, &sensor.beacon);
        let row_distance = sensor.position.y.abs_diff(row);
        if beacon_distance >= row_distance {
            // count positions in relevant row
            let vert_distance = (beacon_distance - row_distance) as i32;
            let row_start = sensor.position.x - vert_distance;
            let row_end = sensor.position.x + vert_distance;
            ranges.push(row_start..=row_end);
        }
    }

    let ranges = merge_intervals(&mut ranges);

    let point_count: usize = ranges.into_iter().map(|range| range.count()).sum();

    point_count - beacons_in_row
}

fn merge_intervals(ranges: &mut [RangeInclusive<i32>]) -> Vec<RangeInclusive<i32>> {
    ranges.sort_by_key(|range| *range.start());

    let mut result: Vec<RangeInclusive<i32>> = vec![ranges[0].clone()];

    for range in ranges.iter_mut().skip(1) {
        let last_range = result.pop().unwrap();
        if range.start() <= last_range.end() {
            let new_end = last_range.end().max(range.end());
            result.push(*last_range.start()..=*new_end);
        } else {
            result.push(last_range);
            result.push(range.clone());
        }
    }

    result
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
