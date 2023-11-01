#![warn(clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::implicit_hasher
)]

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
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;

pub mod queue;

use std::{env, fs, path::Path};

use ureq::get;

pub fn fetch_input(day: i32) {
    let filename = format!("input/day{day:0>#2}.txt");

    if Path::new(&filename).exists() {
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
