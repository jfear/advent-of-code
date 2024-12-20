use day16::part1_path::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../../../input_day16.txt");
    let result = process(file).context("process part 1")?;
    println!("Part 1: {}", result);
    Ok(())
}
