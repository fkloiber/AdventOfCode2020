use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let input = aoc2020::get_puzzle_input(1)?;
    let input: Vec<usize> = input
        .lines()
        .map(|l| l.trim().parse())
        .collect::<Result<_, _>>()?;

    let mut answer = None;
    'outer: for (i, a) in input.iter().enumerate() {
        for b in input[i..].iter() {
            if a + b == 2020 {
                answer = Some(a * b);
                break 'outer;
            }
        }
    }

    let answer = answer.ok_or("answer not found")?;
    aoc2020::check_answer(1, 1, answer)?;

    let mut answer = None;
    'outer_2: for (i, a) in input.iter().enumerate() {
        for (j, b) in input[i..].iter().enumerate() {
            for c in input[(i + j)..].iter() {
                if a + b + c == 2020 {
                    answer = Some(a * b * c);
                    break 'outer_2;
                }
            }
        }
    }

    let answer = answer.ok_or("answer not found")?;
    aoc2020::check_answer(1, 2, answer)?;

    Ok(())
}
