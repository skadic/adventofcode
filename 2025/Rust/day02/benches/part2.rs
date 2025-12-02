fn main() {
    divan::main();
}


#[divan::bench]
fn part2() {
    day02::part2::process().unwrap();
}

