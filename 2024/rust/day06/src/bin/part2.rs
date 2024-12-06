use day06::part2::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let mut file = include_str!("../../../../input_day06.txt");
    let result = process(&mut file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}
