use dotenv;
use std::{env, fs, path::Path};

pub fn input(day: i32) {
    let filename = format!("./src/day{day}/input.txt");
    let file_exists = Path::new(&filename).exists();
    if !file_exists {
        let cookie = {
            dotenv::dotenv().ok();
            let cookie_key = env::var("AOC_COOKIE").expect("cookie exists");
            format!("session={}", cookie_key)
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
