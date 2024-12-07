use day07::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!("../../../input_day07.txt"))).unwrap();
}

#[divan::bench]
fn part2() {
    todo!();
    // part2::process(divan::black_box(include_str!("../../../input_day07.txt"))).unwrap();
}
