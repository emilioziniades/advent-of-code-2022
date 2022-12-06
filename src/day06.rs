use std::{collections::HashSet, fs};

pub fn find_start_marker_position(file: &str, window_size: usize) -> usize {
    let packet = fs::read_to_string(file).expect("file exists");
    packet
        .as_bytes()
        .windows(window_size)
        .position(|packet| packet.len() == packet.iter().collect::<HashSet<_>>().len())
        .unwrap()
        + window_size
}

#[cfg(test)]
mod tests {
    use crate::{day06, fetch_input};

    #[test]
    fn find_packet_start() {
        fetch_input(6);

        let packet_marker_start_size = 4;
        let tests = vec![("example/day06.txt", 7), ("input/day06.txt", 1210)];

        for test in tests {
            let (file, want) = test;
            let got = day06::find_start_marker_position(file, packet_marker_start_size);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }

    #[test]
    fn find_message_start() {
        fetch_input(6);

        let message_marker_start_size = 14;
        let tests = vec![("example/day06.txt", 19), ("input/day06.txt", 3476)];

        for test in tests {
            let (file, want) = test;
            let got = day06::find_start_marker_position(file, message_marker_start_size);
            assert_eq!(got, want, "got {got}, wanted {want}")
        }
    }
}
