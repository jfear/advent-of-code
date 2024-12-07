use {{crate_name}}::part1::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../../../input_{{crate_name}}.txt");
    let result = process(file).context("process part 1")?;
    println!("Part 1: {}", result);
    Ok(())
}
