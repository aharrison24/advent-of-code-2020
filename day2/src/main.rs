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
struct Policy {
    min: i32,
    max: i32,
    c: char,
    password: String,
}

fn to_i32(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse()
}

fn integer(input: &str) -> IResult<&str, i32> {
    map_res(digit1, to_i32)(input)
}

fn policy(input: &str) -> IResult<&str, Policy> {
    let (input, min) = integer(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, max) = integer(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, c) = anychar(input)?;
    let (input, _) = tag(": ")(input)?;
    let password = input.to_string();

    let p = Policy {
        min,
        max,
        c,
        password,
    };
    Ok(("", p))
}

fn is_valid(policy: &Policy) -> bool {
    let count = policy.password.matches(policy.c).count() as i32;
    count >= policy.min && count <= policy.max
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

    let policies: Vec<Policy> = lines
        .iter()
        .map(|x| policy(&x))
        .filter_map(Result::ok)
        .map(|x| x.1)
        .collect_vec();

    let num_valid = policies.iter().filter(|&x| is_valid(x)).count();
    println!("{} valid passwords", num_valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            policy("6-9 d: dddddkzdl"),
            Ok((
                "",
                Policy {
                    min: 6,
                    max: 9,
                    c: 'd',
                    password: String::from("dddddkzdl")
                }
            ))
        );
    }
}
