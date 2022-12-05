pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

use std::{env, fs, path::Path};

pub fn fetch_input(day: i32) {
    let filename = format!("input/day{day:0>#2}.txt");
    let file_exists = Path::new(&filename).exists();
    if !file_exists {
        let cookie = {
            dotenv::dotenv().ok();
            let cookie = env::var("AOC_COOKIE").expect("cookie exists");
            format!("session={cookie}")
        };

        let response = {
            let url = format!("https://adventofcode.com/2022/day/{day}/input");
            let client = reqwest::blocking::Client::new();
            client
                .get(url)
                .header("Cookie", cookie)
                .send()
                .unwrap()
                .text()
                .unwrap()
        };

        fs::write(filename, response).unwrap();
    }
}
