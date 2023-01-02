use std::{cmp, cmp::Ordering, fs};

#[derive(Debug, Clone, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

impl Packet {
    fn from(line: &str) -> Self {
        let mut tokens = tokenize(line);
        let mut root = Packet::List(Vec::new());

        tokens.pop();

        parse(&mut tokens, &mut root);

        root
    }

    fn partial_cmp_recursive(&self, other: &Self, exit: &mut bool, order: &mut Option<Ordering>) {
        if *exit {
            return;
        }
        match (self, other) {
            (Packet::List(left), Packet::List(right)) => {
                let max_length = cmp::max(left.len(), right.len());
                for i in 0..max_length {
                    if *exit {
                        return;
                    }

                    let left = left.get(i);
                    let right = right.get(i);

                    match (left, right) {
                        (Some(left), Some(right)) => left.partial_cmp_recursive(right, exit, order),
                        (Some(_left), None) => {
                            // right ran out of items first
                            *order = Some(Ordering::Greater);
                            *exit = true;
                            break;
                        }
                        (None, Some(_right)) => {
                            // left ran out of items first
                            *order = Some(Ordering::Less);
                            *exit = true;
                            break;
                        }
                        (None, None) => panic!("exceeded both left and right vec lengths"),
                    }
                }
            }

            (Packet::Number(left), Packet::Number(right)) => match left.cmp(right) {
                Ordering::Less => {
                    *order = Some(Ordering::Less);
                    *exit = true;
                }
                Ordering::Greater => {
                    *order = Some(Ordering::Greater);
                    *exit = true;
                }
                Ordering::Equal => {}
            },

            (Packet::List(_), Packet::Number(_)) => {
                let other = &Packet::List(vec![other.clone()]);
                self.partial_cmp_recursive(other, exit, order)
            }

            (Packet::Number(_), Packet::List(_)) => {
                let left = &Packet::List(vec![self.clone()]);
                left.partial_cmp_recursive(other, exit, order)
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut exit = false;
        let mut order = None;
        self.partial_cmp_recursive(other, &mut exit, &mut order);
        order
    }
}

fn parse(tokens: &mut Vec<String>, list: &mut Packet) {
    if let Packet::List(list) = list {
        while let Some(next) = tokens.pop() {
            let next = next.as_str();
            match next {
                "[" => {
                    let mut new_list = Packet::List(Vec::new());
                    parse(tokens, &mut new_list);
                    list.push(new_list);
                }
                "]" => {
                    break;
                }
                n => {
                    let n = Packet::Number(n.parse().unwrap());
                    list.push(n);
                }
            }
        }
    }
}

fn tokenize(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();

    let mut current_number = String::new();

    for chr in line.chars() {
        match chr {
            '[' => tokens.push(chr.to_string()),
            ']' => {
                if !current_number.is_empty() {
                    tokens.push(current_number.clone());
                    current_number.clear();
                }
                tokens.push(chr.to_string())
            }
            ',' => {
                if !current_number.is_empty() {
                    tokens.push(current_number.clone());
                    current_number.clear();
                }
            }
            n => current_number.push(n),
        }
    }
    tokens.reverse(); // so that we can pop off front
    tokens
}

pub fn sum_ordered_pairs(filename: &str) -> usize {
    let packet_pairs = fs::read_to_string(filename).unwrap();
    packet_pairs
        .trim_end()
        .split("\n\n")
        .into_iter()
        .zip(1..packet_pairs.len() + 1)
        .map(|(pair, index)| {
            let (left, right) = {
                let (left, right) = pair.split_once('\n').unwrap();
                (Packet::from(left), Packet::from(right))
            };

            if left < right {
                index
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day13;
    use crate::fetch_input;

    #[test]
    fn sum_ordered_pairs() {
        fetch_input(13);
        let tests = vec![("example/day13.txt", 13), ("input/day13.txt", 6187)];

        for test in tests {
            let (filename, want) = test;
            let got = day13::sum_ordered_pairs(filename);

            assert_eq!(got, want, "got {got}, wanted {want}, for {filename}");
        }
    }
}
