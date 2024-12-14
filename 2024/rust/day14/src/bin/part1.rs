use day14::part1::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let mut file = include_str!("../../../../input_day14.txt");
    let result = process(&mut file, 101, 103).context("process part 1")?;
    println!("Part 1: {}", result);
    Ok(())
}
