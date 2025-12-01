use crate::{INPUT, parse_input};


#[tracing::instrument(name = "part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let input = parse_input(INPUT);

    let mut count = 0usize;
    input.into_iter().fold(50isize, |acc, next| {
        if acc == 0 && next < 0 {
            count -= 1
        }

        if (acc + next).rem_euclid(100) == 0 && next < 0 {
            count += 1
        }

        let diff = acc.div_euclid(100).abs_diff((acc + next).div_euclid(100));
        count += diff;

        (acc + next).rem_euclid(100)
    });

    tracing::info!(count);
    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
