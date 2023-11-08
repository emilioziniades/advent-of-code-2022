use std::fs;

trait BalancedQuinary {
    fn to_balanced_quinary(self) -> String;
}

impl BalancedQuinary for i64 {
    fn to_balanced_quinary(self) -> String {
        if self == 0 {
            return String::new();
        }

        match self % 5 {
            0 => (self / 5).to_balanced_quinary() + "0",
            1 => (self / 5).to_balanced_quinary() + "1",
            2 => (self / 5).to_balanced_quinary() + "2",
            3 => ((self + 2) / 5).to_balanced_quinary() + "=",
            4 => ((self + 1) / 5).to_balanced_quinary() + "-",
            _ => unreachable!("n % 5 is always E [0,4]"),
        }
    }
}

fn balanced_quinary_to_decimal(balanced_quinary_n: &str) -> i64 {
    balanced_quinary_n
        .chars()
        .rev()
        .map(|char| match char {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("unrecognized SNAFU digit"),
        })
        .enumerate()
        .map(|(i, n)| n * 5_i64.pow(i.try_into().unwrap()))
        .sum()
}

pub fn sum_fuel_requirements(input: &str) -> String {
    fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(balanced_quinary_to_decimal)
        .sum::<i64>()
        .to_balanced_quinary()
}

#[cfg(test)]
mod tests {
    use crate::{
        day25::{self, BalancedQuinary},
        fetch_input,
    };

    #[test]
    fn sum_fuel_requirements() {
        fetch_input(25);
        let tests = vec![
            ("example/day25.txt", "2=-1=0"),
            ("input/day25.txt", "2=222-2---22=1=--1-2"),
        ];

        for (filename, want) in tests {
            let got = day25::sum_fuel_requirements(filename);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }

    #[test]
    fn balanced_quinary_conversion() {
        let tests: Vec<(i64, &str)> = vec![
            (1, "1"),
            (2, "2"),
            (3, "1="),
            (4, "1-"),
            (5, "10"),
            (6, "11"),
            (7, "12"),
            (8, "2="),
            (9, "2-"),
            (10, "20"),
            (15, "1=0"),
            (20, "1-0"),
            (2022, "1=11-2"),
            (12345, "1-0---0"),
            (314159265, "1121-1110-1=0"),
        ];
        for (n, balanced_quinary_n) in tests {
            assert_eq!(n.to_balanced_quinary(), balanced_quinary_n);
        }
    }
}
