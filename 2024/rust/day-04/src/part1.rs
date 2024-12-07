use itertools::Itertools;

fn adj_search(input: &Vec<Vec<char>>, i: isize, j: isize) -> u32 {
  let kernel = [
    [(i, j), (i, j + 1), (i, j + 2), (i, j + 3)],
    [(i, j), (i + 1, j + 1), (i + 2, j + 2), (i + 3, j + 3)],
    [(i, j), (i + 1, j), (i + 2, j), (i + 3, j)],
    [(i, j), (i + 1, j - 1), (i + 2, j - 2), (i + 3, j - 3)],
    [(i, j), (i, j - 1), (i, j - 2), (i, j - 3)],
    [(i, j), (i - 1, j - 1), (i - 2, j - 2), (i - 3, j - 3)],
    [(i, j), (i - 1, j), (i - 2, j), (i - 3, j)],
    [(i, j), (i - 1, j + 1), (i - 2, j + 2), (i - 3, j + 3)],
  ];
  let mut count = 0;
  let dims = (input.len() as isize, input[0].len() as isize);
  for indices in kernel {
    dbg!(indices);
    dbg!(dims);
    if indices
      .iter()
      .any(|(i, j)| *i < 0 || *j < 0 || *i >= input.len() as isize || *j >= input[0].len() as isize)
    {
      continue;
    }
    if input[indices[0].0 as usize][indices[0].1 as usize] == 'X'
      && input[indices[1].0 as usize][indices[1].1 as usize] == 'M'
      && input[indices[2].0 as usize][indices[2].1 as usize] == 'A'
      && input[indices[3].0 as usize][indices[3].1 as usize] == 'S'
    {
      count += 1;
    }
  }
  count
}

#[tracing::instrument]
pub fn process(input: &str) -> anyhow::Result<String> {
  let input = input.lines().map(|s| s.chars().collect_vec()).collect_vec();

  let mut count = 0;
  for (i, line) in input.iter().enumerate() {
    for (j, c) in line.iter().cloned().enumerate() {
      if c == 'X' {
        count += adj_search(&input, i as isize, j as isize);
      }
    }
  }

  Ok(count.to_string())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> anyhow::Result<()> {
    let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    assert_eq!("18", process(input)?);
    Ok(())
  }
}
