use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

pub fn get_puzzle_input(day: u8) -> Result<String, Box<dyn Error>> {
    if day == 0 || day > 25 {
        Err(format!("day out of range"))?;
    }

    if let Ok(mut file) = File::open(format!("input/day{}.txt", day)) {
        let mut input = String::new();
        file.read_to_string(&mut input)?;
        return Ok(input);
    }

    let session = std::env::var("AOC_SESSION_COOKIE")?;
    let client = reqwest::blocking::Client::new();
    let input = client
        .get(&format!("https://adventofcode.com/2020/day/{}/input", day))
        .header("cookie", format!("session={}", session))
        .send()?
        .text()?;

    {
        std::fs::create_dir_all("input")?;
        let mut file = File::create(format!("input/day{}.txt", day))?;
        file.write_all(input.as_bytes())?;
    }

    Ok(input)
}
