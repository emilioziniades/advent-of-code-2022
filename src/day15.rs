use std::{cmp::Ordering, fs, ops::RangeInclusive};

const CHARACTERS: [char; 17] = [
    'S', 'e', 'n', 's', 'o', 'r', 'a', 't', 'c', 'l', 'b', 'i', 'x', 'y', '=', ':', ',',
];

#[derive(Debug, Clone, Copy)]
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
        .count();

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

    ranges = remove_overlapping_ranges(&mut ranges);

    println!("{ranges:?}");

    let point_count: usize = ranges.into_iter().map(|range| range.count()).sum();

    point_count - beacons_in_row - 2
}

fn manhattan_distance(a: &Point, b: &Point) -> u32 {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

fn remove_overlapping_ranges(ranges: &mut Vec<RangeInclusive<i32>>) -> Vec<RangeInclusive<i32>> {
    ranges.sort_by(compare_ranges);
    println!("{ranges:?}");
    ranges.reverse();

    let mut result: Vec<RangeInclusive<i32>> = Vec::new();

    while let Some((first, second)) = ranges.pop().zip(ranges.pop()) {
        // first wholy contains second
        if first.start() < second.start() && first.end() > second.end() {
            result.push(first);
        }
        // second's start is in first
        else if first.start() < second.start() && first.end() > second.start() {
            result.push(*first.start()..=*second.start());
            result.push(*second.start() + 1..=*first.end());
            result.push(*first.end() + 1..=*second.end());
        }
        // no overlap
        else {
            result.push(first);
            result.push(second);
        }
    }

    result
}

fn compare_ranges(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> Ordering {
    a.start().cmp(b.start())
}

#[cfg(test)]
mod tests {
    use crate::{day15, fetch_input};

    #[test]
    fn count_non_beacons() {
        fetch_input(15);

        let tests = vec![
            ("example/day15.txt", 26, 10),
            // ("input/day15.txt", 5_125_700, 2_000_000),
        ];

        for test in tests {
            let (file, want, row) = test;
            let got = day15::count_non_beacons(file, row);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }
}
