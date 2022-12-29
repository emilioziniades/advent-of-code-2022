use std::fs;
// use std::{cmp::Ordering, fs};

#[derive(Debug, PartialEq, PartialOrd)]
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
    /*
    fn recursive_partial_cmp(
        &self,
        other: &Packet,
        order: &mut Option<Ordering>,
        escape: &mut bool,
    ) {
        if *escape {
            return;
        }
        match (self, other) {
            (Packet::Number(s), Packet::Number(o)) => {
                order = &mut s.partial_cmp(o).clone();
                match order {
                    Some(Ordering::Greater) | Some(Ordering::Less) => {
                        *escape = true;
                        return;
                    }
                    _ => (),
                }
            }
            (Packet::List(s), Packet::List(o)) => {
                for (index, item) in s.iter().enumerate() {
                    item.recursive_partial_cmp(&o[index], order, escape)
                }
            }
            (Packet::List(_s), Packet::Number(_o)) => todo!(),
            (Packet::Number(_s), Packet::List(_o)) => todo!(),
        }
    }
    */
}

/*
impl PartialEq for Packet {
    fn eq(&self, _other: &Self) -> bool {
        // packets will never be exactly equal
        false
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut order = None;
        let mut escape = false;
        self.recursive_partial_cmp(other, &mut order, &mut escape);
        println!("{order:?}");
        order
    }
}
*/

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
    tokens.reverse(); // so that we can "pop" off front
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
                let (left, right) = dbg!(pair.split_once('\n').unwrap());
                (dbg!(Packet::from(left)), dbg!(Packet::from(right)))
                // (Packet::from(left), Packet::from(right))
            };

            if left < right {
                println!("index {index}: left < right");
                index
            } else {
                println!("not less");
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
        let tests = vec![("example/day13.txt", 13) /*("input/day13.txt", 0)*/];

        for test in tests {
            let (filename, want) = test;
            let got = day13::sum_ordered_pairs(filename);

            assert_eq!(got, want, "got {got}, wanted {want}, for {filename}");
        }
    }
}
