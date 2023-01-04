use std::{collections::HashSet, fs};

const CHARACTERS: [char; 17] = [
    'S', 'e', 'n', 's', 'o', 'r', 'a', 't', 'c', 'l', 'b', 'i', 'x', 'y', '=', ':', ',',
];

const MULTIPLIER: isize = 4_000_000;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy)]
pub struct Interval {
    start: isize,
    end: isize,
}

impl Interval {
    fn new(start: isize, end: isize) -> Self {
        Interval { start, end }
    }
}

#[derive(Debug)]
struct Sensor {
    position: Point,
    beacon: Point,
}

pub fn count_non_beacons(file: &str, row: isize) -> isize {
    let sensors = parse_sensors(file);

    let ranges = get_non_beacon_ranges(&sensors, row);

    let beacons_in_row: isize = sensors
        .iter()
        .filter(|sensor| sensor.beacon.y == row)
        .map(|sensor| sensor.beacon)
        .collect::<HashSet<Point>>()
        .len()
        .try_into()
        .unwrap();

    let points_count: isize = ranges.iter().map(|range| range.end - range.start + 1).sum();

    points_count - beacons_in_row
}

pub fn find_distress_beacon(file: &str, interval: Interval) -> usize {
    let sensors = parse_sensors(file);

    for y in interval.start..=interval.end {
        let ranges = get_non_beacon_ranges(&sensors, y);

        let points_count: isize = ranges
            .iter()
            // truncate to desired range
            .map(
                |range| match (range.start < interval.start, range.end > interval.end) {
                    (true, true) => Interval::new(interval.start, interval.end),
                    (true, false) => Interval::new(interval.start, range.end),
                    (false, true) => Interval::new(range.start, interval.end),
                    (false, false) => *range,
                },
            )
            .map(|range| range.end - range.start + 1)
            .sum();

        if points_count != interval.end + 1 {
            // distress beacon in this row!
            let x = find_excluded_points(ranges);
            return (x * MULTIPLIER + y) as usize;
        }
    }

    panic!("did not find distress beacon")
}

fn find_excluded_points(ranges: Vec<Interval>) -> isize {
    assert_eq!(ranges.len(), 2, "should have two large intervals");
    assert_eq!(
        ranges[0].end + 1,
        ranges[1].start - 1,
        "intervals should have single gap"
    );

    ranges[0].end + 1
}

fn parse_sensors(file: &str) -> Vec<Sensor> {
    let input = fs::read_to_string(file).unwrap();
    input
        .lines()
        .map(|line| {
            let numbers = line
                .replace(CHARACTERS, "")
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<isize>>();

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
        .collect()
}

fn get_non_beacon_ranges(sensors: &Vec<Sensor>, row: isize) -> Vec<Interval> {
    let mut ranges: Vec<Interval> = Vec::new();

    for sensor in sensors {
        // does it reach row
        let beacon_distance = manhattan_distance(&sensor.position, &sensor.beacon);
        let row_distance = sensor.position.y.abs_diff(row);
        if beacon_distance >= row_distance {
            // count positions in relevant row
            let vert_distance = (beacon_distance - row_distance) as isize;
            let row_start = sensor.position.x - vert_distance;
            let row_end = sensor.position.x + vert_distance;
            ranges.push(Interval::new(row_start, row_end));
        }
    }

    merge_intervals(&mut ranges)
}

fn merge_intervals(ranges: &mut [Interval]) -> Vec<Interval> {
    ranges.sort_by_key(|range| range.start);

    let mut result: Vec<Interval> = vec![ranges[0]];

    for range in ranges.iter().skip(1) {
        let last_range = result.pop().unwrap();
        if range.start <= last_range.end {
            let new_end = last_range.end.max(range.end);
            result.push(Interval::new(last_range.start, new_end));
        } else {
            result.push(last_range);
            result.push(*range);
        }
    }

    result
}

fn manhattan_distance(a: &Point, b: &Point) -> usize {
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

    #[test]
    fn find_distress_beacon() {
        fetch_input(15);

        let tests = vec![
            ("example/day15.txt", 56_000_011, day15::Interval::new(0, 20)),
            (
                "input/day15.txt",
                11_379_394_658_764,
                day15::Interval::new(0, 4_000_000),
            ),
        ];

        for test in tests {
            let (file, want, interval) = test;
            let got = day15::find_distress_beacon(file, interval);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }
}
