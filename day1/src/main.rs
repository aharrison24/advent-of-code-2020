use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let file = File::open(&args.path).expect("Cannot open file");

    let nums = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .filter_map(|x| x.parse::<i32>().ok())
        .collect_vec();

    let combos = nums.iter().combinations(2);

    let mut prod = combos
        .filter(|x| x.iter().cloned().sum::<i32>() == 2020)
        .map(|x| x.iter().cloned().product::<i32>());

    match prod.next() {
        None => println!("No answer found!"),
        Some(x) => println!("{}", x),
    }
}
