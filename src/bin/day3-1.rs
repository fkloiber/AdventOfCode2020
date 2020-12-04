use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let input: Vec<Vec<u8>> = aoc2020::get_puzzle_input(3)?
        .lines()
        .map(str::trim)
        .map(Into::into)
        .collect();

    let mut pos = 0;
    let mut trees = 0;

    for line in input.iter() {
        if line[pos] == b'#' {
            trees += 1;
        }
        pos = (pos + 3) % line.len();
    }

    aoc2020::check_answer(3, 1, trees)?;

    Ok(())
}
