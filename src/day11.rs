use std::{collections::VecDeque, fs, iter};

struct Monkey {
    items: VecDeque<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test_divisor: usize,
    test_true_monkey: usize,
    test_false_monkey: usize,
}

impl Monkey {
    fn from(lines: &str) -> Self {
        /* Expects input in this form
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3
         */
        let mut lines = lines.lines().skip(1); // skip "Monkey n:"

        let items = lines.next().expect("another line");
        let items: VecDeque<usize> = items
            .trim()
            .strip_prefix("Starting items: ")
            .expect("starts with Starting items:")
            .replace(',', "")
            .split_whitespace()
            .map(|n| n.parse().expect("a number"))
            .collect();

        let operation = lines
            .next()
            .expect("another line")
            .trim()
            .strip_prefix("Operation: new = old ")
            .expect("starts with Operation:")
            .split_whitespace()
            .collect::<Vec<&str>>();

        let operator = operation[0];
        let operand = operation[1].parse().unwrap_or(0);

        let operation: Box<dyn Fn(usize) -> usize> = match (operator, operand) {
            ("*", 0) => Box::new(|old| old * old),
            ("+", 0) => Box::new(|old| old + old),
            ("*", n) => Box::new(move |old| old * n),
            ("+", n) => Box::new(move |old| old + n),
            _ => panic!("unrecognized operation"),
        };

        let test_divisor: usize = lines
            .next()
            .expect("another line")
            .trim()
            .strip_prefix("Test: divisible by ")
            .expect("starts with Test: divisible by")
            .parse()
            .expect("a number");

        let test_true_monkey: usize = lines
            .next()
            .expect("another line")
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .expect("starts with If true: throw to monkey")
            .parse()
            .expect("a number");

        let test_false_monkey: usize = lines
            .next()
            .expect("another line")
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .expect("starts with If false: throw to monkey")
            .parse()
            .expect("a number");

        Monkey {
            items,
            operation,
            test_divisor,
            test_true_monkey,
            test_false_monkey,
        }
    }
}

struct Monkeys {
    m: Vec<Monkey>,
    count: Vec<usize>,
    prime_product: usize,
}

impl Monkeys {
    fn from(filename: &str) -> Self {
        let monkeys: Vec<Monkey> = fs::read_to_string(filename)
            .unwrap()
            .split("\n\n")
            .map(Monkey::from)
            .collect();

        let n_monkeys = monkeys.len();

        let prime_product = monkeys
            .iter()
            .map(|m| m.test_divisor)
            .reduce(|a, b| a * b)
            .unwrap();

        Self {
            m: monkeys,
            count: iter::repeat(0).take(n_monkeys).collect(),
            prime_product,
        }
    }

    fn do_rounds(&mut self, n: usize, worry_management: WorryManagement) {
        match worry_management {
            WorryManagement::DivByThree => {
                for _ in 0..n {
                    self.do_round_div_three();
                }
            }
            WorryManagement::ModProductPrimes => {
                for _ in 0..n {
                    self.do_round_mod_prime();
                }
            }
        }
    }

    fn do_round_div_three(&mut self) {
        for i in 0..self.m.len() {
            while let Some(old_item) = self.m[i].items.pop_front() {
                self.count[i] += 1;

                let mut new_item = (self.m[i].operation)(old_item);

                new_item /= 3;

                let target_monkey_id = match new_item % self.m[i].test_divisor == 0 {
                    true => self.m[i].test_true_monkey,
                    false => self.m[i].test_false_monkey,
                };

                self.m[target_monkey_id].items.push_back(new_item);
            }
        }
    }

    fn do_round_mod_prime(&mut self) {
        for i in 0..self.m.len() {
            while let Some(old_item) = self.m[i].items.pop_front() {
                self.count[i] += 1;

                let mut new_item = (self.m[i].operation)(old_item);

                new_item %= self.prime_product;

                let target_monkey_id = match new_item % self.m[i].test_divisor == 0 {
                    true => self.m[i].test_true_monkey,
                    false => self.m[i].test_false_monkey,
                };

                self.m[target_monkey_id].items.push_back(new_item);
            }
        }
    }
}

pub enum WorryManagement {
    DivByThree,
    ModProductPrimes,
}

pub fn measure_monkey_business(
    filename: &str,
    n_rounds: usize,
    worry_management: WorryManagement,
) -> usize {
    let mut monkeys = Monkeys::from(filename);
    monkeys.do_rounds(n_rounds, worry_management);
    monkeys.count.sort_by(|a, b| b.cmp(a));
    monkeys.count[0] * monkeys.count[1]
}

#[cfg(test)]
mod tests {
    use crate::day11;
    use crate::fetch_input;

    #[test]
    fn measure_monkey_business() {
        fetch_input(11);
        let tests = vec![
            (
                "example/day11.txt",
                20,
                day11::WorryManagement::DivByThree,
                10605,
            ),
            (
                "input/day11.txt",
                20,
                day11::WorryManagement::DivByThree,
                58056,
            ),
            (
                "example/day11.txt",
                10_000,
                day11::WorryManagement::ModProductPrimes,
                2713310158,
            ),
            (
                "input/day11.txt",
                10_000,
                day11::WorryManagement::ModProductPrimes,
                15048718170,
            ),
        ];

        for test in tests {
            let (filename, n_rounds, worry_management, want) = test;
            let got = day11::measure_monkey_business(filename, n_rounds, worry_management);

            assert_eq!(got, want, "got {got}, wanted {want}, for {filename}");
        }
    }
}
