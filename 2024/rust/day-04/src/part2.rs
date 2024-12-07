use itertools::Itertools;

fn index(input: &Vec<Vec<char>>, (i, j): (isize, isize)) -> char {
  input[i as usize][j as usize]
}

fn check_match(input: &Vec<Vec<char>>, indices: [(isize, isize); 3]) -> bool {
  index(input, indices[0]) == 'M'
    && index(input, indices[1]) == 'A'
    && index(input, indices[2]) == 'S'
}

fn adj_search(input: &Vec<Vec<char>>, i: isize, j: isize) -> bool {
  let kernel = [
    [(i - 1, j - 1), (i, j), (i + 1, j + 1)],
    [(i + 1, j + 1), (i, j), (i - 1, j - 1)],
    [(i - 1, j + 1), (i, j), (i + 1, j - 1)],
    [(i + 1, j - 1), (i, j), (i - 1, j + 1)],
  ];
  (check_match(input, kernel[0]) || check_match(input, kernel[1]))
    && (check_match(input, kernel[2]) || check_match(input, kernel[3]))
}

#[tracing::instrument]
pub fn process(input: &str) -> anyhow::Result<String> {
  let input = input.lines().map(|s| s.chars().collect_vec()).collect_vec();

  let mut count = 0;
  for (i, line) in input.iter().enumerate() {
    if i == 0 || i == input.len() - 1 {
      continue;
    }
    for (j, c) in line.iter().cloned().enumerate() {
      if j == 0 || j == line.len() - 1 {
        continue;
      }
      if c == 'A' {
        count += adj_search(&input, i as isize, j as isize) as u32;
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

    assert_eq!("9", process(input)?);
    Ok(())
  }
}
