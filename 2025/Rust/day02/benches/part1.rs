fn main() {
    divan::main();
}


#[divan::bench]
fn part1() {
    day02::part1::process().unwrap();
}
