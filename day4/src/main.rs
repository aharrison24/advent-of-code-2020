use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

type Passport = HashMap<String, String>;

fn reduce_passport_fields(mut acc: Passport, field_string: &str) -> Passport {
    let mut iter = field_string.split(':');
    let (field, value) = (iter.next().unwrap(), iter.next().unwrap());
    acc.insert(field.to_string(), value.to_string());
    acc
}

fn check_validity_for_part1(passport: &Passport) -> bool {
    const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    REQUIRED_FIELDS.iter().all(|&x| passport.contains_key(x))
}

fn main() {
    let args = Cli::from_args();

    let file = File::open(&args.path).expect("Cannot open file");
    let lines = BufReader::new(file).lines().filter_map(Result::ok);
    let passport_strings = gather_passport_strings(lines);

    let passports = passport_strings
        .iter()
        .map(|x| parse_passport(x))
        .collect_vec();

    println!("Total passports: {}", passports.iter().count());
    let num_valid_part_1 = passports.iter().filter(|&x| check_validity_for_part1(x)).count();
    println!("Part 1: Number of valid passports = {}", num_valid_part_1);
}

fn parse_passport(input: &str) -> Passport {
    input.split_ascii_whitespace().fold(Passport::new(), reduce_passport_fields)
}

fn gather_passport_strings<'a, I>(lines: I) -> Vec<String>
where
    I: IntoIterator<Item = String>,
{
    let mut passport_data = Vec::new();
    for (key, mut group) in &lines.into_iter().group_by(|x| !x.is_empty()) {
        if key {
            passport_data.push(group.join(" "));
        }
    }
    passport_data
}