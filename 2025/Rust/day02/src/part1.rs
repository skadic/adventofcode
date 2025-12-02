use tracing::info;

use crate::{INPUT, SAMPLE, double_up, find_count_start, parse_input};

#[tracing::instrument(name = "part1", parent=None)]
pub fn process() -> miette::Result<()> {
    let input = parse_input(INPUT);

    let mut count = 0usize;

    for range in input {
        
        let mut n = find_count_start(*range.start());
        let mut v = double_up(n);
        while v <= *range.end() {
            if range.contains(&v) {
                count += v;
            }
            n += 1;
            v = double_up(n);
        }
    }

    info!(count);

    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
