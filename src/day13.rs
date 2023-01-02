use std::{cmp, cmp::Ordering, fs};

#[derive(Debug, Clone)]
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
    tokens.reverse(); // so that we can "pop" off front
    tokens
}

fn compare(left: &Packet, right: &Packet) -> Ordering {
    let mut exit = false;
    let mut order = Ordering::Equal;
    compare_recursive(left, right, &mut exit, &mut order);

    println!("\n>>>>>>>>>>>>>>>>>>>>>>>>>  {order:?}\n");

    order
}
fn compare_recursive(left: &Packet, right: &Packet, exit: &mut bool, order: &mut Ordering) {
    if *exit {
        return;
    }
    match (left, right) {
        (Packet::List(left), Packet::List(right)) => {
            let max_length = cmp::max(left.len(), right.len());
            for i in 0..max_length {
                if *exit {
                    return;
                }

                let left = dbg!(left.get(i));
                let right = dbg!(right.get(i));

                match (left, right) {
                    (Some(left), Some(right)) => compare_recursive(left, right, exit, order),
                    (Some(_left), None) => {
                        // right ran out of items first
                        *order = Ordering::Greater;
                        *exit = true;
                        break;
                    }
                    (None, Some(_right)) => {
                        // left ran out of items first
                        *order = Ordering::Less;
                        *exit = true;
                        break;
                    }
                    (None, None) => panic!("exceeded both left and right vec lengths"),
                }
            }
        }
        (Packet::Number(l), Packet::Number(r)) => match dbg!(dbg!(l).cmp(dbg!(r))) {
            Ordering::Equal => {}
            Ordering::Less => {
                *order = Ordering::Less;
                *exit = true;
            }
            Ordering::Greater => {
                *order = Ordering::Greater;
                *exit = true;
            }
        },
        (Packet::List(_), Packet::Number(_)) => {
            let right = &Packet::List(vec![right.clone()]);
            compare_recursive(left, right, exit, order)
        }
        (Packet::Number(_), Packet::List(_)) => {
            let left = &Packet::List(vec![left.clone()]);
            compare_recursive(left, right, exit, order)
        }
    }
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
                (dbg!(Packet::from(left)), dbg!(Packet::from(right)))
                // (Packet::from(left), Packet::from(right))
            };

            dbg!(match compare(&left, &right) {
                Ordering::Less => index,
                Ordering::Greater => 0,
                Ordering::Equal => panic!("packets will never be equal"),
            })
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
