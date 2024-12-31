use day20::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(
        divan::black_box(include_str!("../../../input_day20.txt")),
        100,
    )
    .unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!("../../../input_day20.txt"))).unwrap();
}
