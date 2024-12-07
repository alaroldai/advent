use day_04::part1::process;
use anyhow::Context;

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input1.txt");
    let result = process(file).context("process part 1")?;
    println!("{}", result);
    Ok(())
}
