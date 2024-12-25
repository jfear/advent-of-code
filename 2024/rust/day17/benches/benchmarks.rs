use day17::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!(
        "../../../input_day17.txt"
    )))
    .unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!(
        "../../../input_day17.txt"
    )))
    .unwrap();
}
