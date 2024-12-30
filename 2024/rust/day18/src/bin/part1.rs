use day18::part1::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let mut file = include_str!("../../../../input_day18.txt");
    let result = process(&mut file, 1024, 70).context("process part 1")?;
    println!("Part 1: {}", result);
    Ok(())
}
