use nom::{
  bytes::complete::tag,
  character::complete::one_of,
  combinator::{map_res, recognize},
  multi::many_m_n,
  sequence::{delimited, separated_pair},
  IResult,
};

struct MultiplyExpression(i32, i32);

fn parse_number(input: &str) -> IResult<&str, i32> {
  map_res(
    recognize(many_m_n(1, 3, one_of("1234567890"))),
    |out: &str| i32::from_str_radix(out, 10),
  )(input)
}

fn parse_expr(input: &str) -> IResult<&str, MultiplyExpression> {
  map_res(
    delimited(
      tag("mul("),
      separated_pair(parse_number, tag(","), parse_number),
      tag(")"),
    ),
    |(a, b)| -> Result<MultiplyExpression, ()> { Ok(MultiplyExpression(a, b)) },
  )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<MultiplyExpression>> {
  map_res(parse_expr, |exp| -> Result<Vec<MultiplyExpression>, ()> {
    Ok(vec![exp])
  })(input)
}

#[tracing::instrument]
pub fn process(expr: &str) -> anyhow::Result<String> {
  let mut result = 0;
  for i in 0..expr.len() {
    if let Ok((remainder, MultiplyExpression(a, b))) = parse_expr(expr.split_at(i).1) {
      result += a * b;
    }
  }
  Ok(result.to_string())
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("mul(44,46)", 2024)]
  #[case("mul(123,4)", 492)]
  #[case("mul(4*", 0)]
  #[case("mul(6,9!", 0)]
  #[case("?(12,34)", 0)]
  #[case("mul ( 2 , 4 )", 0)]
  #[case(
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    161
  )]
  fn test_process(#[case] expr: &str, #[case] result: i32) -> anyhow::Result<()> {
    assert_eq!(result.to_string(), process(expr)?);
    Ok(())
  }
}
