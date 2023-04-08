use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Number {
    Shifted(isize),
    Unshifted(isize),
}

impl Number {
    fn to_shifted(self) -> Self {
        match self {
            Number::Unshifted(n) => Number::Shifted(n),
            Number::Shifted(_) => panic!("we have already shifted this item"),
        }
    }
}

impl From<Number> for isize {
    fn from(value: Number) -> Self {
        match value {
            Number::Shifted(n) => n,
            Number::Unshifted(n) => n,
        }
    }
}

#[derive(Debug)]
struct Numbers(Vec<Number>);

impl Numbers {
    fn move_item(&mut self, from_index: usize, to_index: usize) {
        let item = self.0[from_index];
        let item = item.to_shifted();
        self.0.remove(from_index);
        self.0.insert(to_index, item);
    }

    fn next_to_shift(&self) -> Option<usize> {
        self.0
            .iter()
            .position(|n| matches!(n, Number::Unshifted(_)))
    }
}

pub fn sum_coordinates(infile: &str) -> isize {
    let numbers: Vec<Number> = fs::read_to_string(infile)
        .unwrap()
        .lines()
        .map(|line| Number::Unshifted(line.parse().unwrap()))
        .collect();

    let mut numbers = Numbers(numbers);
    let n_numbers = numbers.0.len();

    while let Some(index) = numbers.next_to_shift() {
        println!("{numbers:?}");
        let number: isize = numbers.0[index].into();
        let uindex: isize = index.try_into().unwrap();
        let n_numbers: isize = n_numbers.try_into().unwrap();

        let new_index = (uindex + number).rem_euclid(n_numbers).try_into().unwrap();
        println!("moving {number} from {index} to {new_index}");
        numbers.move_item(index, new_index);
    }

    let zero_position = numbers
        .0
        .iter()
        .position(|n| n == &Number::Shifted(0))
        .unwrap();

    let coordinates = {
        let x = (zero_position + 1000) % n_numbers;
        let y = (zero_position + 2000) % n_numbers;
        let z = (zero_position + 3000) % n_numbers;

        [numbers.0[x], numbers.0[y], numbers.0[z]]
    };

    return coordinates.into_iter().map(isize::from).sum();
}

#[cfg(test)]
mod tests {
    use crate::{day20, fetch_input};

    #[test]
    fn surface_area() {
        fetch_input(20);
        let tests = vec![
            ("example/day20.txt", 3), /* ("input/day18.txt", 3412) */
        ];

        for (infile, want) in tests {
            let got = day20::sum_coordinates(infile);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }
}
