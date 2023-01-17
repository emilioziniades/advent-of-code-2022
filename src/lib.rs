pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;

pub mod pathfind;
pub mod queue;

use std::{env, fs, path::Path};

use ureq::get;

pub fn fetch_input(day: i32) {
    let filename = format!("input/day{day:0>#2}.txt");
    let file_exists = Path::new(&filename).exists();

    if file_exists {
        return;
    }

    let cookie = {
        dotenv::dotenv().ok();
        let cookie = env::var("AOC_COOKIE").expect("cookie exists");
        format!("session={cookie}")
    };

    let response = {
        let url = format!("https://adventofcode.com/2022/day/{day}/input");
        get(&url)
            .set("Cookie", &cookie)
            .call()
            .expect("call succeeds")
            .into_string()
            .expect("body is parsed")
    };

    fs::write(filename, response).unwrap();
}
