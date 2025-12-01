fn main() {
    divan::main();
}


#[divan::bench]
fn part2() {
    day01::part2::process().unwrap();
}

