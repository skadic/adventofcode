use crate::{INPUT, parse_input};

#[tracing::instrument(name = "part1", parent=None)]
pub fn process() -> miette::Result<()> {
    let input = parse_input(INPUT);

    let mut count = 0usize;
    input.into_iter().fold(50isize, |acc, next| {
        if (acc + next).rem_euclid(100) == 0 {
            count += 1;
        };
        acc + next 
    });

    tracing::info!(count);
    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
