use itertools::Itertools;

pub fn levels_ok(levels: impl IntoIterator<Item = i32>) -> bool {
  let mut last_diff = 0;
  for (a, b) in levels.into_iter().tuple_windows() {
    let diff = b - a;
    if diff * last_diff < 0 {
      return false;
    }
    if diff.abs() < 1 || diff.abs() > 3 {
      return false;
    }
    last_diff = diff;
  }
  return true;
}

pub fn levels_ok_with_dampening(levels: &[i32]) -> bool {
  if levels_ok(levels.iter().cloned()) {
    return true;
  }

  // this is a little sad... I wonder if we could do this with sliding windows?
  for i in 0..levels.len() {
    if levels_ok(
      levels
        .iter()
        .enumerate()
        .filter(|(j, _)| *j != i)
        .map(|(_, l)| *l),
    ) {
      return true;
    }
  }
  return false;
}

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
      .map(|levels| -> u32 { levels_ok_with_dampening(&levels).into() })
      .sum::<u32>()
      .to_string(),
  )
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(&[7, 6, 4, 2, 1,], true)]
  #[case(&[1, 2, 7, 8, 9,], false)]
  #[case(&[9, 7, 6, 2, 1,], false)]
  #[case(&[1, 3, 2, 4, 5,], true)]
  #[case(&[8, 6, 4, 4, 1,], true)]
  #[case(&[1, 3, 6, 7, 9,], true)]
  #[case(&[89, 91, 94, 96, 97, 99, 98, 98,], false)]
  fn test_simple(#[case] levels: &[i32], #[case] expected: bool) {
    assert_eq!(levels_ok_with_dampening(levels), expected);
  }

  #[test]
  fn test_process() -> anyhow::Result<()> {
    let input = r#"
      7 6 4 2 1
      1 2 7 8 9
      9 7 6 2 1
      1 3 2 4 5
      8 6 4 4 1
      1 3 6 7 9
      89 91 94 96 97 99 98 98
    "#;
    assert_eq!("4", process(input)?);
    Ok(())
  }
}
