use {{crate_name}}::part2::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../../../input_{{crate_name}}.txt");
    let result = process(file).context("process part 2")?;
    println!("Part 2: {}", result);
    Ok(())
}
