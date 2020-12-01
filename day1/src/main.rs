use std::collections::HashSet;
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

    let nums: HashSet<_> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    for num in &nums {
        if nums.contains(&(2020 - num)) {
            println!("Result: {}", num * (2020 - num));
            return;
        }
    }

    println!("No result found");
}
