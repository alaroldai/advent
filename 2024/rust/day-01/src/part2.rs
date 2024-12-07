use itertools::Itertools;
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> anyhow::Result<String> {
  Ok({
    let mut xs = HashMap::<i64, i64>::new();
    let mut ys = HashMap::<i64, i64>::new();
    for (x, y) in input
      .lines()
      .filter_map(|line| line.split_whitespace().next_tuple())
    {
      *xs.entry(x.parse().unwrap()).or_insert(0i64) += 1;
      *ys.entry(y.parse().unwrap()).or_insert(0i64) += 1;
    }
    xs.iter()
      .map(|(key, count)| key * count * ys.get(key).cloned().unwrap_or(0))
      .sum::<i64>()
      .to_string()
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> anyhow::Result<()> {
    let input = r#"
      3   4
      4   3
      2   5
      1   3
      3   9
      3   3
    "#;
    assert_eq!("31", process(input)?);
    Ok(())
  }
}
