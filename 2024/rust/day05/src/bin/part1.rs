use day05::part1::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../../../input_day05.txt");
    let result = process(file).context("process part 1")?;
    println!("{}", result);
    Ok(())
}
