use anyhow::Context;
use day_01::part2::process;

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt::init();

  let file = include_str!("../../input2.txt");
  let result = process(file).context("process part 2")?;
  println!("{}", result);
  Ok(())
}
