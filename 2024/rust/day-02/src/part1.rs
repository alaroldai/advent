use itertools::Itertools;
use tracing::info;

#[tracing::instrument]
pub fn process(input: &str) -> anyhow::Result<String> {
  Ok(
    input
      .lines()
      .map(|l| {
        l.split_whitespace()
          .map(|s| s.parse::<i32>().unwrap())
          .collect::<Vec<i32>>()
      })
      .filter(|l| !l.is_empty())
      .map(|levels| -> u32 {
        let mut last_diff = 0;
        for i in 1..levels.len() {
          let diff = levels[i] - levels[i - 1];
          if diff * last_diff < 0 {
            return 0u32;
          }
          if diff.abs() < 1 || diff.abs() > 3 {
            return 0u32;
          }
          last_diff = diff;
        }
        return 1u32;
      })
      .sum::<u32>()
      .to_string(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> anyhow::Result<()> {
    let input = r#"
      7 6 4 2 1
      1 2 7 8 9
      9 7 6 2 1
      1 3 2 4 5
      8 6 4 4 1
      1 3 6 7 9
    "#;
    assert_eq!("2", process(input)?);
    Ok(())
  }
}
