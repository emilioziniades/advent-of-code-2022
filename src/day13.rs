use std::fs;

const LBRACE: &str = "[";
const RBRACE: &str = "]";

#[derive(Debug)]
#[allow(dead_code)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

impl PacketPair {
    fn from(lines: &str) -> Self {
        let (left, right) = lines.split_once('\n').unwrap();
        Self {
            left: Packet::from(left.trim()),
            right: Packet::from(right.trim()),
        }
    }
}

#[derive(Debug)]
enum Packet {
    List(Vec<Packet>),
    Number(usize),
}

impl Packet {
    fn from(line: &str) -> Self {
        let tokens = Self::tokenize(line);
        Self::parse(tokens)
    }

    fn tokenize(line: &str) -> Vec<String> {
        println!("{line}");
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
        tokens
    }

    fn parse(tokens: Vec<String>) -> Self {
        println!("{tokens:?}");
        let mut stack: Vec<String> = Vec::new();

        for token in tokens {
            match token.as_str() {
                LBRACE => stack.push(token),
                RBRACE => {
                    let mut list: Vec<Packet> = Vec::new();
                    loop {
                        let token = stack.pop().unwrap();
                        if token == LBRACE {
                            break;
                        }
                        let number: usize = token.parse().expect("a number");
                        list.push(Packet::Number(number));
                    }

                    list.reverse();
                    // todo!();
                    println!("{list:?}")
                }
                _ => stack.push(token),
            }
        }

        Self::List(Vec::new())
    }
}

pub fn sum_ordered_pairs(filename: &str) -> usize {
    let packet_pairs = fs::read_to_string(filename).unwrap();
    let packet_pairs: Vec<PacketPair> = packet_pairs.split("\n\n").map(PacketPair::from).collect();
    println!("{packet_pairs:#?}");
    1
}

#[cfg(test)]
mod tests {
    use crate::day13;
    use crate::fetch_input;

    #[test]
    fn sum_ordered_pairs() {
        fetch_input(13);
        let tests = vec![("example/day13.txt", 1) /*("input/day12.txt", 462)*/];

        for test in tests {
            let (filename, want) = test;
            let got = day13::sum_ordered_pairs(filename);

            assert_eq!(got, want, "got {got}, wanted {want}, for {filename}");
        }
    }
}
