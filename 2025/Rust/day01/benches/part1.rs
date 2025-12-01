fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    day01::part1::process().unwrap();
}
