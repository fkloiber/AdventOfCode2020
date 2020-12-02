use std::error::Error;

#[derive(Debug)]
struct InputLine {
    min: usize,
    max: usize,
    char: char,
    pw: String,
}

fn parse_input_line(input: &str) -> Option<InputLine> {
    use nom::{
        bytes::complete::tag,
        character::complete::{anychar, digit1},
        combinator::{map_res, rest},
        sequence::tuple,
    };
    let (_, (min, _, max, _, char, _, pw)) = tuple::<_, _, nom::error::Error<&str>, _>((
        map_res(digit1, str::parse::<usize>),
        tag("-"),
        map_res(digit1, str::parse::<usize>),
        tag(" "),
        anychar,
        tag(": "),
        rest,
    ))(input)
    .ok()?;
    Some(InputLine {
        min,
        max,
        char,
        pw: pw.into(),
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let input = aoc2020::get_puzzle_input(2)?;
    let input: Vec<InputLine> = input
        .lines()
        .map(|l| parse_input_line(l.trim()))
        .collect::<Option<_>>()
        .ok_or("error parsing input")?;

    fn pw_valid(l: &&InputLine) -> bool {
        let equal =
            l.pw.chars()
                .enumerate()
                .filter(|(i, c)| (l.min - 1 == *i || l.max - 1 == *i) && *c == l.char)
                .count();
        equal == 1
    }
    let answer = input.iter().filter(pw_valid).count();

    aoc2020::check_answer(2, 2, answer)?;

    Ok(())
}
