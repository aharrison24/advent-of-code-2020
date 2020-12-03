use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::map_res,
    IResult,
};
use std::fs::File;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;

#[derive(Debug, PartialEq)]
struct PolicyAndPassword {
    num1: usize,
    num2: usize,
    character: char,
    password: String,
}

fn parse_positive_integer(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |x: &str| x.parse())(input)
}

fn parse_policy(input: &str) -> IResult<&str, PolicyAndPassword> {
    let (input, num1) = parse_positive_integer(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, num2) = parse_positive_integer(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, c) = anychar(input)?;
    let (input, _) = tag(": ")(input)?;
    let password = input.to_string();

    let p = PolicyAndPassword {
        num1,
        num2,
        character: c,
        password,
    };
    Ok(("", p))
}

fn is_valid_for_part1(policy: &PolicyAndPassword) -> bool {
    let count = policy.password.matches(policy.character).count();
    count >= policy.num1 && count <= policy.num2
}

fn is_valid_for_part2(policy: &PolicyAndPassword) -> bool {
    let char_indices = [policy.num1, policy.num2];
    let extracted_chars = char_indices
        .iter()
        .map(|&x| policy.password.chars().nth(x - 1).unwrap());
    let num_matches = extracted_chars.filter(|&x| x == policy.character).count();
    num_matches == 1
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let file = File::open(&args.path).expect("Cannot open file");

    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect_vec();

    let policies: Vec<PolicyAndPassword> = lines
        .iter()
        .map(|x| parse_policy(&x))
        .filter_map(Result::ok)
        .map(|x| x.1)
        .collect_vec();

    let num_valid_part1 = policies.iter().filter(|&x| is_valid_for_part1(x)).count();
    println!("Part 1 valid passwords: {}", num_valid_part1);

    let num_valid_part2 = policies.iter().filter(|&x| is_valid_for_part2(x)).count();
    println!("Part 2 valid passwords: {}", num_valid_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            parse_policy("6-9 d: dddddkzdl"),
            Ok((
                "",
                PolicyAndPassword {
                    num1: 6,
                    num2: 9,
                    character: 'd',
                    password: String::from("dddddkzdl")
                }
            ))
        );
    }
}
