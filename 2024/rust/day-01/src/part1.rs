use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> anyhow::Result<String> {
  Ok({
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for (x, y) in input
      .lines()
      .filter_map(|line| line.split_whitespace().next_tuple())
    {
      xs.push(x);
      ys.push(y);
    }
    xs.sort();
    ys.sort();
    xs.iter()
      .zip(ys.iter())
      .map(|(x, y)| {
        let (x, y): (i64, i64) = (x.parse().unwrap(), y.parse().unwrap());
        (x - y).abs()
      })
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
    assert_eq!("11", process(input)?);
    Ok(())
  }
}
