use tracing::info;

use crate::{is_repeated, parse_input, INPUT};

#[tracing::instrument(name = "part2", parent=None)]
pub fn process() -> miette::Result<()> {
    let input = parse_input(INPUT);

    let result = input
        .into_iter()
        .flatten()
        .filter(|&val| is_repeated(val))
        .sum::<usize>();

    info!(result);

    Ok(())
}

#[cfg(test)]
#[test]
fn part2() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    process()
}
