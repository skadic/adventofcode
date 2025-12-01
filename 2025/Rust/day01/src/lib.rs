pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

pub mod part1;
pub mod part2;

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<isize> {
    input
    .lines()
    .map(|line| line.replace('L', "-").replace('R', "+").parse().unwrap())
    .collect()
}

