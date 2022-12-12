use std::fs;

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn from(line: &str) -> Self {
        let line: Vec<&str> = line.split_whitespace().collect();
        match line[..] {
            ["noop"] => Self::Noop,
            ["addx", n] => Self::AddX(n.parse().unwrap()),
            [..] => panic!("unrecognized instruction"),
        }
    }

    fn cycle_length(&self) -> u8 {
        match self {
            Self::Noop => 1,
            Self::AddX(_) => 2,
        }
    }
}

struct Cpu {
    cycles: i32,
    signal_strengths: Vec<i32>,
    x: i32,
}

impl Cpu {
    fn new() -> Self {
        Self {
            cycles: 0,
            signal_strengths: Vec::new(),
            x: 1,
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        for _ in 0..instruction.cycle_length() {
            self.cycles += 1;

            if (self.cycles - 20) % 40 == 0 {
                self.signal_strengths.push(self.cycles * self.x)
            }
        }
        if let Instruction::AddX(addx) = instruction {
            self.x += addx
        }
    }
}

pub fn sum_signal_strengths(filename: &str) -> i32 {
    let mut cpu = Cpu::new();

    let instructions = fs::read_to_string(filename).unwrap();
    let instructions = instructions.lines();

    for instruction in instructions {
        let instruction = Instruction::from(instruction);
        cpu.execute(instruction);
    }

    cpu.signal_strengths.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::day10;
    use crate::fetch_input;

    #[test]
    fn count_tail_positions() {
        fetch_input(10);
        let tests = vec![
            ("example/day10.txt", 13140),
            ("input/day10.txt", 11820),
            // ("input/day09.txt", 6181, 2),
            // ("example/day09.txt", 1, 10),
            // ("example/day09_2.txt", 36, 10),
            // ("input/day09.txt", 2386, 10),
        ];

        for test in tests {
            let (filename, want) = test;
            let got = day10::sum_signal_strengths(filename);

            assert_eq!(got, want, "got {got}, wanted {want}, for {filename}");
        }
    }
}
