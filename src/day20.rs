use std::{collections::VecDeque, fs};

const DECRYPTION_KEY: isize = 811_589_153;

struct Number {
    value: isize,
    id: usize,
}

impl Number {
    fn new(value: isize, id: usize) -> Self {
        Self { value, id }
    }
}

struct Numbers(VecDeque<Number>);

impl Numbers {
    fn from_file(filename: &str) -> Self {
        let numbers_list = fs::read_to_string(filename)
            .unwrap()
            .lines()
            .enumerate()
            .map(|(id, line)| Number::new(line.parse().unwrap(), id))
            .collect();

        Self(numbers_list)
    }

    // this preserves the relative ordering of items in the list
    // but does not preserve the absolute indexes of the items.
    // It doesn't matter since the solution is based on values relative to
    // zero.
    fn shift_item(&mut self, index: usize) {
        let item = self.0.remove(index).unwrap();
        let delta = isize::try_from(index).unwrap() + item.value;
        let new_index = delta.rem_euclid(isize::try_from(self.0.len()).unwrap());
        self.0.insert(usize::try_from(new_index).unwrap(), item);
    }

    // this is the bottleneck - it takes O(n) to find
    // the next index to shift.
    fn next_index_to_shift(&self, id: usize) -> usize {
        self.0.iter().position(|n| n.id == id).unwrap()
    }

    fn mix(&mut self) {
        for i in 0..self.0.len() {
            let index = self.next_index_to_shift(i);
            self.shift_item(index);
        }
    }

    fn coordinates(&self) -> [isize; 3] {
        let zero_position = self.0.iter().position(|n| n.value == 0).unwrap();
        let n_numbers = self.0.len();
        let x = (zero_position + 1000) % n_numbers;
        let y = (zero_position + 2000) % n_numbers;
        let z = (zero_position + 3000) % n_numbers;
        [self.0[x].value, self.0[y].value, self.0[z].value]
    }

    fn apply_decryption_key(&mut self, decryption_key: isize) {
        self.0 = self
            .0
            .iter()
            .map(|number| Number::new(number.value * decryption_key, number.id))
            .collect();
    }
}

pub fn mix_once(filename: &str) -> isize {
    let mut numbers = Numbers::from_file(filename);
    numbers.mix();
    numbers.coordinates().iter().sum()
}

pub fn mix_ten_times(filename: &str) -> isize {
    let mut numbers = Numbers::from_file(filename);
    numbers.apply_decryption_key(DECRYPTION_KEY);

    for _ in 0..10 {
        numbers.mix();
    }

    numbers.coordinates().iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::{day20, fetch_input};

    #[test]
    fn mix_once() {
        fetch_input(20);

        let tests = vec![("example/day20.txt", 3), ("input/day20.txt", 872)];

        for (infile, want) in tests {
            let got = day20::mix_once(infile);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }

    #[test]
    fn mix_ten_times() {
        fetch_input(20);

        let tests = vec![
            ("example/day20.txt", 1623178306),
            ("input/day20.txt", 5382459262696),
        ];

        for (infile, want) in tests {
            let got = day20::mix_ten_times(infile);
            assert_eq!(got, want, "got {got}, wanted {want}");
        }
    }
}
