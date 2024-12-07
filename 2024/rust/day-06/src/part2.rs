use bitmask::bitmask;
use core::str;
use std::{collections::HashMap, fmt::Display};

use glam::{i16vec2, I16Vec2};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

bitmask! {
  mask GridCell: u8 where flags Element {
    North = 1,
    East = 1 << 1,
    South = 1 << 2,
    West = 1 << 3,
    Obstacle = 1 << 4,
  }
}

impl From<u8> for GridCell {
  fn from(value: u8) -> Self {
    match value {
      b'.' => GridCell::none(),
      b'#' => GridCell::from(Element::Obstacle),
      b'^' => GridCell::from(Element::North),
      b'>' => GridCell::from(Element::East),
      b'v' => GridCell::from(Element::South),
      b'<' => GridCell::from(Element::West),
      _ => panic!(),
    }
  }
}

impl Default for GridCell {
  fn default() -> Self {
    GridCell::none()
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Hash)]
struct Guard {
  location: I16Vec2,
  direction: I16Vec2,
}

impl Guard {
  pub fn direction_element(&self) -> Element {
    match (self.direction.x, self.direction.y) {
      (1, 0) => Element::East,
      (0, 1) => Element::South,
      (-1, 0) => Element::West,
      (0, -1) => Element::North,
      _ => panic!(),
    }
  }
}

#[derive(Clone)]
pub struct Scene {
  grid: Vec<GridCell>,
  dims: I16Vec2,
  guard: Guard,
}

impl Scene {
  fn at(&self, loc: I16Vec2) -> GridCell {
    self.grid[loc.y as usize * self.dims.x as usize + loc.x as usize]
  }
  fn at_mut(&mut self, loc: I16Vec2) -> &mut GridCell {
    &mut self.grid[loc.y as usize * self.dims.x as usize + loc.x as usize]
  }

  fn contains(&self, loc: I16Vec2) -> bool {
    (0..self.dims.x).contains(&loc.x) && (0..self.dims.y).contains(&loc.y)
  }
}

fn load_map(input: &str) -> Scene {
  let lines = input.lines().collect_vec();
  let mut grid = vec![GridCell::none(); lines.len() * lines[0].len()];

  let mut guard = Guard::default();
  for (i, line) in lines.iter().enumerate() {
    for (j, c) in line.bytes().enumerate() {
      grid[i * line.len() + j] = GridCell::from(c);
      if c == b'^' {
        guard = Guard {
          location: i16vec2(j as i16, i as i16),
          direction: i16vec2(0, -1),
        };
      }
    }
  }
  Scene {
    grid,
    dims: i16vec2(lines[0].len() as i16, lines.len() as i16),
    guard,
  }
}

#[derive(Debug)]
pub enum ExitReason {
  LeftScene,
  Loop,
}

// #[tracing::instrument(skip(scene))]
pub fn search(scene: &mut Scene, obstacle: I16Vec2) -> ExitReason {
  scene.at_mut(obstacle).set(Element::Obstacle);

  let result = loop {
    let next = scene.guard.location + scene.guard.direction;
    if !scene.contains(next) {
      break ExitReason::LeftScene;
    }
    let next = match scene.at(next) {
      cell if cell.contains(Element::Obstacle) => Guard {
        location: scene.guard.location,
        direction: scene.guard.direction.perp(),
      },
      _ => {
        let guard = Guard {
          location: next,
          ..scene.guard
        };
        guard
      }
    };
    if scene.at(next.location).contains(next.direction_element()) {
      break ExitReason::Loop;
    }

    scene.guard = next;
    scene.at_mut(next.location).set(next.direction_element());
  };

  return result;
}

// #[tracing::instrument(skip(input))]
pub fn process(input: &str) -> anyhow::Result<String> {
  let mut scene = load_map(input);

  // let mut preceeders: FxHashMap<I16Vec2, Guard> =
  //   FxHashMap::with_capacity_and_hasher(10000, FxBuildHasher::default());
  let mut preceeders = HashMap::with_capacity(10000);

  loop {
    let next = scene.guard.location + scene.guard.direction;
    if !scene.contains(next) {
      break;
    }

    let next = match scene.at(next) {
      cell if cell.contains(Element::Obstacle) => Guard {
        location: scene.guard.location,
        direction: scene.guard.direction.perp(),
      },
      _ => {
        let guard = Guard {
          location: next,
          ..scene.guard
        };
        guard
      }
    };

    preceeders.entry(next.location).or_insert(scene.guard);
    scene.guard = next;
  }

  Ok(
    preceeders
      .into_par_iter()
      .filter_map(move |(point, from)| -> Option<I16Vec2> {
        let mut scene = Scene {
          grid: scene.grid.clone(),
          dims: scene.dims,
          guard: from,
        };
        if let ExitReason::Loop = search(&mut scene, point) {
          return Some(point);
        }
        return None;
      })
      .count()
      .to_string(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load_map() -> anyhow::Result<()> {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    load_map(&input);
    Ok(())
  }

  #[test]
  fn test_process() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    assert_eq!("6", process(input)?);
    Ok(())
  }
}

impl Display for GridCell {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {
      mask if mask == GridCell::none() => write!(f, "."),
      mask if mask == GridCell::from(Element::North) => write!(f, "^"),
      mask if mask == GridCell::from(Element::East) => write!(f, ">"),
      mask if mask == GridCell::from(Element::South) => write!(f, "v"),
      mask if mask == GridCell::from(Element::West) => write!(f, "<"),
      mask if mask == GridCell::from(Element::Obstacle) => write!(f, "#"),
      mask if mask == Element::North | Element::South => write!(f, "|"),
      mask if mask == Element::East | Element::West => write!(f, "-"),
      mask if !mask.contains(Element::Obstacle) => {
        write!(f, "+")
      }
      _ => panic!("No string conversion for {:b}", self.mask),
    }
  }
}

impl Display for Scene {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for line in 0..self.dims.y {
      for col in 0..self.dims.x {
        write!(
          f,
          "{}",
          self.grid[line as usize * self.dims.x as usize + col as usize]
        )?;
      }
      writeln!(f, "")?;
    }
    Ok(())
  }
}
