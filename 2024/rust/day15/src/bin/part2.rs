use day15::part2::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let mut file = include_str!("../../../../input_day15.txt");
    let result = process(&mut file).context("process part 2")?;
    println!("Part 2: {}", result);
    Ok(())
}
