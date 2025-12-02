use std::ops::RangeInclusive;

pub const INPUT: &str = include_str!("../input.txt");
pub const SAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

pub const DIVISORS: [&[usize]; 13] = [
    &[],
    &[],
    &[1usize],
    &[1],
    &[1, 2],
    &[1],
    &[1, 2, 3],
    &[1],
    &[1, 2, 4],
    &[1, 3],
    &[1, 2, 5],
    &[1],
    &[1, 2, 3, 4, 6]
];


pub mod part1;
pub mod part2;

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<RangeInclusive<usize>> {
    input
        .split(',')
        .map(|range| range.split_once('-').unwrap())
        .map(|(l, r)| l.parse().unwrap()..=r.parse().unwrap())
        .collect()
}

fn is_repeated(n: usize) -> bool {
    let divisors = DIVISORS[num_digits(n)];
    let n_str = n.to_string().into_bytes();

    for &divisor in divisors {
        let mut chunks = n_str.chunks(divisor);
        let first = chunks.next().unwrap();
        if chunks.all(|cnk| cnk == first) {
            return true
        }
    }
    false
}

fn num_digits(n: usize) -> usize {
    (n as f64 + 1.0).log10().ceil() as usize
}

pub fn find_count_start(lower_bound: usize) -> usize {
    if lower_bound < 10 {
        return 0;
    }

    let mut num_digits = num_digits(lower_bound);
    if !num_digits.is_multiple_of(2) {
        num_digits += 1
    }

    10usize.pow(num_digits as u32 / 2 - 1)
}

pub fn double_up(n: usize) -> usize {
    10usize.pow(num_digits(n) as u32) * n + n
}

#[cfg(test)]
mod test {
    use crate::{double_up, is_repeated};

    #[test]
    fn test_double_up() {
        assert_eq!(double_up(10), 1010);
        assert_eq!(double_up(4), 44);
        assert_eq!(double_up(1512), 15121512);
        assert_eq!(double_up(75431), 7543175431);
        assert_eq!(double_up(444), 444444);
    }


    #[test]
    fn test_is_repeated() {
        assert!(is_repeated(444));
        assert!(is_repeated(12431243));
        assert!(!is_repeated(512342));
        assert!(is_repeated(5151515151));
        assert!(!is_repeated(6622422));
        assert!(is_repeated(1051910519));
    }
}
