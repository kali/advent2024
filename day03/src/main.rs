use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{anychar, digit1};
use nom::combinator::map;
use nom::multi::fold_many1;
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;

fn mul(input: &str) -> IResult<&str, usize> {
    map(
        delimited(
            tag("mul("),
            separated_pair(number, tag(","), number),
            tag(")"),
        ),
        |(a, b)| a * b,
    )(input)
}

fn number(input: &str) -> IResult<&str, usize> {
    map(digit1, |s: &str| s.parse::<usize>().unwrap())(input)
}

fn p1(input: &str) -> IResult<&str, usize> {
    fold_many1(alt((mul, map(anychar, |_| 0))), || 0, |a, b| a + b)(input)
}

fn p2(input: &str) -> IResult<&str, usize> {
    fold_many1(
        alt((
            mul,
            map(preceded(tag("don't()"), take_until("do()")), |_| 0),
            map(preceded(tag("don't()"), take_until("do()")), |_| 0),
            map(anychar, |_| 0),
        )),
        || 0,
        |a, b| a + b,
    )(input)
}

fn main() -> anyhow::Result<()> {
    let mut input = std::fs::read_to_string("input")?.trim().to_string();
    input.push_str("do()");
    dbg!(p1(&input).unwrap().1);
    dbg!(p2(&input).unwrap().1);
    Ok(())
}

#[test]
fn a() {
    assert_eq!(p1("mul(4,5)").unwrap().1, 20);
    assert_eq!(p1("abcdmul(4,5)").unwrap().1, 20);
}

#[test]
fn b() {
    assert_eq!(p2("don't()   don't() mul(1,1) do() mul(2,1)").unwrap().1, 2);
    assert_eq!(p2("don't()   do() mul(1,1) do() mul(2,1)").unwrap().1, 3);
}
