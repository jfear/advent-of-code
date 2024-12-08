use day08::part2::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let mut file = include_str!("../../../../input_day08.txt");
    let result = process(&mut file).context("process part 1")?;
    println!("Part 1: {}", result);
    Ok(())
}
