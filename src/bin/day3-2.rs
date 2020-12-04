use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let input: Vec<Vec<u8>> = aoc2020::get_puzzle_input(3)?
        .lines()
        .map(str::trim)
        .map(Into::into)
        .collect();

    let mut answer = 1;

    answer *= check_slope(&input, 1, 1);
    answer *= check_slope(&input, 3, 1);
    answer *= check_slope(&input, 5, 1);
    answer *= check_slope(&input, 7, 1);
    answer *= check_slope(&input, 1, 2);

    aoc2020::check_answer(3, 2, answer)?;

    Ok(())
}

fn check_slope(map: &[Vec<u8>], run: usize, rise: usize) -> usize {
    let is = (0..).map(|i| i * rise).take_while(|i| *i < map.len());
    let mut pos = 0;
    let mut trees = 0;

    for i in is {
        let line = &map[i];
        if line[pos] == b'#' {
            trees += 1;
        }
        pos = (pos + run) % line.len();
    }

    trees
}
