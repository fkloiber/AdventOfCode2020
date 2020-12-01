use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let input = advent_of_code_2020::get_puzzle_input(1)?;
    let input: Vec<u32> = input
        .lines()
        .map(|l| l.trim().parse())
        .collect::<Result<_, _>>()?;

    println!("{:#?}", input);

    Ok(())
}
