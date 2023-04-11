use std::{collections::VecDeque, fs};

struct Number {
    has_shifted: bool,
    value: isize,
}

struct Numbers(VecDeque<Number>);

impl Numbers {
    fn from_file(filename: &str) -> Self {
        let numbers_list = fs::read_to_string(filename)
            .unwrap()
            .lines()
            .map(|line| Number {
                value: line.parse().unwrap(),
                has_shifted: false,
            })
            .collect();

        Self(numbers_list)
    }
    fn shift_item(&mut self, index: usize) {
        let mut item = self.0.remove(index).unwrap();
        let delta = index as isize + item.value;
        let to_index = delta.rem_euclid(self.0.len() as isize);
        item.has_shifted = true;
        self.0.insert(to_index as usize, item);
    }

    fn next_to_shift(&self) -> Option<usize> {
        self.0.iter().position(|n| !n.has_shifted)
    }

    fn mix(&mut self) {
        while let Some(index) = self.next_to_shift() {
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
}

pub fn mix_once(filename: &str) -> isize {
    let mut numbers = Numbers::from_file(filename);
    numbers.mix();
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
}
