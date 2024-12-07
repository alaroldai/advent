use anyhow::anyhow;
use nom::bytes::complete::tag;
use nom::character::complete::*;
use nom::combinator::{map, map_res, recognize};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

fn correctly_ordered(rules: &[PageOrderingRule], list: &[u32]) -> bool {
  // dumb implementation first
  for i in 0..list.len() {
    for j in i..list.len() {
      for r in rules {
        if r.a == list[j] && r.b == list[i] {
          return false;
        }
      }
    }
  }
  return true;
}

#[tracing::instrument]
pub fn process(input: &str) -> anyhow::Result<String> {
  let (rules, page_lists) = parse_input(input).map_err(|e| anyhow!(e.to_string()))?.1;

  let result: u32 = page_lists
    .iter()
    .filter(|list| correctly_ordered(&rules, &list.0))
    .map(|list| list.0[list.0.len() / 2])
    .sum();

  Ok(result.to_string())
}

#[derive(PartialEq, Eq, Debug)]
struct PageOrderingRule {
  a: u32,
  b: u32,
}

#[derive(PartialEq, Eq, Debug)]
struct PageList(Vec<u32>);

fn u32(input: &str) -> IResult<&str, u32> {
  map_res(recognize(digit1), |nums: &str| nums.parse())(input)
}

impl PageOrderingRule {
  fn parse(input: &str) -> IResult<&str, PageOrderingRule> {
    map(separated_pair(u32, char('|'), u32), |(a, b)| {
      PageOrderingRule { a, b }
    })(input)
  }
}

impl PageList {
  fn parse(input: &str) -> IResult<&str, PageList> {
    map(separated_list1(char(','), u32), |xs| PageList(xs))(input)
  }
}

fn parse_input(input: &str) -> IResult<&str, (Vec<PageOrderingRule>, Vec<PageList>)> {
  separated_pair(
    separated_list1(char('\n'), PageOrderingRule::parse),
    tag("\n\n"),
    separated_list1(char('\n'), PageList::parse),
  )(input)
}

#[cfg(test)]
mod tests {
  use anyhow::anyhow;
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("47|53", PageOrderingRule { a: 47, b: 53 })]
  #[case("97|61", PageOrderingRule { a: 97, b: 61 })]
  #[case("97|47", PageOrderingRule { a: 97, b: 47 })]
  #[case("75|29", PageOrderingRule { a: 75, b: 29 })]
  #[case("61|13", PageOrderingRule { a: 61, b: 13 })]
  #[case("75|53", PageOrderingRule { a: 75, b: 53 })]
  #[case("29|13", PageOrderingRule { a: 29, b: 13 })]
  #[case("97|29", PageOrderingRule { a: 97, b: 29 })]
  #[case("53|29", PageOrderingRule { a: 53, b: 29 })]
  #[case("61|53", PageOrderingRule { a: 61, b: 53 })]
  #[case("97|53", PageOrderingRule { a: 97, b: 53 })]
  #[case("61|29", PageOrderingRule { a: 61, b: 29 })]
  #[case("47|13", PageOrderingRule { a: 47, b: 13 })]
  #[case("75|47", PageOrderingRule { a: 75, b: 47 })]
  #[case("97|75", PageOrderingRule { a: 97, b: 75 })]
  #[case("47|61", PageOrderingRule { a: 47, b: 61 })]
  #[case("75|61", PageOrderingRule { a: 75, b: 61 })]
  #[case("47|29", PageOrderingRule { a: 47, b: 29 })]
  #[case("75|13", PageOrderingRule { a: 75, b: 13 })]
  #[case("53|13", PageOrderingRule { a: 53, b: 13 })]
  #[case("97|13", PageOrderingRule { a: 97, b: 13 })]
  fn test_parse_ordering_rule(
    #[case] input: &str,
    #[case] expected: PageOrderingRule,
  ) -> anyhow::Result<()> {
    let actual = PageOrderingRule::parse(input)
      .map_err(|e| anyhow!(e.to_string()))?
      .1;
    assert_eq!(expected, actual);
    Ok(())
  }

  #[rstest]
  #[case("75,47,61,53,29", &[75,47,61,53,29])]
  #[case("97,61,53,29,13", &[97,61,53,29,13])]
  #[case("75,29,13", &[75,29,13])]
  #[case("75,97,47,61,53", &[75,97,47,61,53])]
  #[case("61,13,29", &[61,13,29])]
  #[case("97,13,75,29,47", &[97,13,75,29,47])]
  fn test_parse_page_list(#[case] input: &str, #[case] expected: &[u32]) -> anyhow::Result<()> {
    assert_eq!(
      expected,
      &PageList::parse(input)
        .map_err(|e| anyhow!(e.to_string()))?
        .1
         .0
    );
    Ok(())
  }

  #[rstest]
  #[case(&[75,47,61,53,29], true)]
  #[case(&[97,61,53,29,13], true)]
  #[case(&[75,29,13], true)]
  #[case(&[75,97,47,61,53], false)]
  #[case(&[61,13,29], false)]
  #[case(&[97,13,75,29,47], false)]
  fn test_rule_application(#[case] pages: &[u32], #[case] expected: bool) {
    let rules = &[
      PageOrderingRule { a: 47, b: 53 },
      PageOrderingRule { a: 97, b: 61 },
      PageOrderingRule { a: 97, b: 47 },
      PageOrderingRule { a: 75, b: 29 },
      PageOrderingRule { a: 61, b: 13 },
      PageOrderingRule { a: 75, b: 53 },
      PageOrderingRule { a: 29, b: 13 },
      PageOrderingRule { a: 97, b: 29 },
      PageOrderingRule { a: 53, b: 29 },
      PageOrderingRule { a: 61, b: 53 },
      PageOrderingRule { a: 97, b: 53 },
      PageOrderingRule { a: 61, b: 29 },
      PageOrderingRule { a: 47, b: 13 },
      PageOrderingRule { a: 75, b: 47 },
      PageOrderingRule { a: 97, b: 75 },
      PageOrderingRule { a: 47, b: 61 },
      PageOrderingRule { a: 75, b: 61 },
      PageOrderingRule { a: 47, b: 29 },
      PageOrderingRule { a: 75, b: 13 },
      PageOrderingRule { a: 53, b: 13 },
      PageOrderingRule { a: 97, b: 13 },
    ];
    assert_eq!(expected, correctly_ordered(rules, pages));
  }

  #[test]
  fn test_process() -> anyhow::Result<()> {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    assert_eq!("143", process(input)?);
    Ok(())
  }
}
